use alloy_primitives::{Address, B256, Bytes, FixedBytes, U256, keccak256, map::HashMap};
use alloy_rlp::encode;
use alloy_trie::{HashBuilder, Nibbles, proof::ProofRetainer};
use forge::revm::primitives::{
    Account, AccountInfo, EvmState, EvmStorageSlot,
    alloy_primitives::private::{alloy_rlp, alloy_rlp::Encodable},
};
use mpt::{Keccak256, reorder_root_first};
use provider::StorageProof;

fn to_nibbles<T: AsRef<[u8]>>(word: T) -> Nibbles {
    Nibbles::unpack(keccak256(word))
}

fn build_proof(
    targets: Vec<Nibbles>,
    leafs: Vec<(Nibbles, Vec<u8>)>,
) -> (B256, HashMap<Nibbles, Bytes>) {
    let mut builder = HashBuilder::default().with_proof_retainer(ProofRetainer::new(targets));

    for (key, value) in leafs {
        builder.add_leaf(key, &value);
    }

    let root = builder.root();

    let proof_nodes = builder.take_proof_nodes().into_inner();
    (root, proof_nodes)
}

pub fn account_proof(address: Address, evm_state: &EvmState) -> Vec<Bytes> {
    let (root, proof_nodes) = build_proof(vec![to_nibbles(address)], trie_accounts(evm_state));
    reorder_root_first::<_, Keccak256>(proof_nodes.values(), root)
        .into_iter()
        .cloned()
        .collect::<Vec<_>>()
}

fn trie_accounts(accounts: &HashMap<Address, Account>) -> Vec<(Nibbles, Vec<u8>)> {
    let mut accounts = accounts
        .iter()
        .map(|(address, account)| {
            (to_nibbles(*address), trie_account_rlp(&account.info, &account.storage))
        })
        .collect::<Vec<_>>();
    accounts.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

    accounts
}

fn trie_account_rlp(info: &AccountInfo, storage: &HashMap<U256, EvmStorageSlot>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let list: [&dyn Encodable; 4] =
        [&info.nonce, &info.balance, &storage_root(storage), &info.code_hash];

    alloy_rlp::encode_list::<_, dyn Encodable>(&list, &mut out);

    out
}

pub fn storage_root(storage: &HashMap<U256, EvmStorageSlot>) -> B256 {
    build_root(trie_storage(storage))
}

fn build_root(values: impl IntoIterator<Item = (Nibbles, Vec<u8>)>) -> B256 {
    let mut builder = HashBuilder::default();
    for (key, value) in values {
        builder.add_leaf(key, value.as_ref());
    }
    builder.root()
}

fn trie_storage(storage: &HashMap<U256, EvmStorageSlot>) -> Vec<(Nibbles, Vec<u8>)> {
    let mut storage = storage
        .iter()
        .map(|(key, value)| (to_nibbles(key.to_be_bytes::<32>()), encode(value.present_value)))
        .collect::<Vec<_>>();
    storage.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

    storage
}

pub fn prove_storage(
    storage: &HashMap<U256, EvmStorageSlot>,
    storage_keys: &[FixedBytes<32>],
) -> Vec<StorageProof> {
    let keys: Vec<_> = storage_keys.iter().map(to_nibbles).collect();

    let (root, all_proof_nodes) = build_proof(keys.clone(), trie_storage(storage));

    let mut proofs = Vec::new();
    for proof_key in keys {
        let matching_proof_nodes = all_proof_nodes
            .iter()
            .filter(|(path, _)| proof_key.starts_with(path))
            .map(|(_, node)| node.clone());
        proofs.push(reorder_root_first::<_, Keccak256>(matching_proof_nodes, root));
    }

    storage_keys
        .iter()
        .zip(proofs)
        .map(|(key, proof)| StorageProof {
            key: *key,
            value: storage
                .get(&(*key).into())
                .cloned()
                .unwrap_or_default()
                .present_value,
            proof,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use alloy_primitives::{U160, address, b256, hex};
    use alloy_rlp::RlpDecodable;
    use mpt::KeccakMerkleTrie as MerkleTrie;
    use serde_json::{Value, from_str, from_value};

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, RlpDecodable)]
    struct StateAccount {
        pub nonce: u64,
        pub balance: U256,
        pub storage_root: B256,
        pub code_hash: B256,
    }

    fn read_and_parse_json_file(file_path: &str) -> Value {
        let file_content = fs::read_to_string(file_path).expect("Failed to read the file");
        from_str(&file_content).expect("Failed to parse JSON from file")
    }

    fn build_state() -> EvmState {
        let json_value = read_and_parse_json_file("testdata/dumped_evm_state.json");
        let evm_state: EvmState = from_value(json_value).expect("Failed to parse EVM state");
        evm_state
    }

    #[test]
    fn test_address_to_nibbles() {
        let address = Address::from(U160::from(0x0012_3456));
        assert_eq!(
            keccak256(address),
            b256!("f5c7e89ecc8b4dced430a51ceb6cac1af1067e6aef60306400853e88334e023c")
        );
        let nibbles = to_nibbles(address);
        assert_eq!(
            nibbles.as_slice(),
            hex!(
                "0f050c070e08090e0c0c080b040d0c0e0d0403000a05010c0e0b060c0a0c010a0f010006070e060a0e0f06000300060400000805030e08080303040e0002030c"
            )
        );
    }

    #[test]
    fn test_account_proof_is_decoded_by_mpt() {
        let evm_state: EvmState = build_state();
        let address = address!("5615deb798bb3e4dfa0139dfa1b3d433cc23b72f");
        let proofs = account_proof(address, &evm_state);
        let mpt = MerkleTrie::from_rlp_nodes(proofs).unwrap();
        let decoded_account = mpt
            .get_rlp::<StateAccount>(keccak256(address))
            .unwrap()
            .unwrap();
        let expected_account = StateAccount {
            nonce: evm_state.get(&address).unwrap().info.nonce,
            balance: evm_state.get(&address).unwrap().info.balance,
            code_hash: evm_state.get(&address).unwrap().info.code_hash,
            storage_root: storage_root(&evm_state.get(&address).unwrap().storage),
        };
        assert_eq!(decoded_account, expected_account);
    }
}
