use alloy_primitives::{Address, Bytes, FixedBytes, B256, U256};
use alloy_trie::proof::ProofRetainer;
use alloy_trie::{HashBuilder, Nibbles};
use ethers_core::utils::keccak256;
use forge::revm::primitives::alloy_primitives::private::alloy_rlp;
use forge::revm::primitives::alloy_primitives::private::alloy_rlp::Encodable;
use forge::revm::primitives::{Account, AccountInfo, EvmState, EvmStorageSlot};
use std::collections::{BTreeMap, HashMap};

fn address_to_nibbles(address: Address) -> Nibbles {
    Nibbles::unpack(keccak256(address))
}

fn build_proof(targets: Vec<Nibbles>, leafs: Vec<(Nibbles, Vec<u8>)>) -> BTreeMap<Nibbles, Bytes> {
    let mut builder = HashBuilder::default().with_proof_retainer(ProofRetainer::new(targets));

    for (key, value) in leafs {
        builder.add_leaf(key, &value);
    }

    let _ = builder.root();

    builder.take_proofs()
}

pub fn account_proof(address: Address, evm_state: &EvmState) -> Vec<Bytes> {
    build_proof(vec![address_to_nibbles(address)], trie_accounts(evm_state))
        .values()
        .cloned()
        .collect::<Vec<_>>()
}

fn trie_accounts(accounts: &HashMap<Address, Account>) -> Vec<(Nibbles, Vec<u8>)> {
    let mut accounts = accounts
        .iter()
        .map(|(address, account)| {
            let data = trie_account_rlp(&account.info, &account.storage);
            (address_to_nibbles(*address), data)
        })
        .collect::<Vec<_>>();
    accounts.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

    accounts
}

fn trie_account_rlp(info: &AccountInfo, storage: &HashMap<U256, EvmStorageSlot>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let list: [&dyn Encodable; 4] = [
        &info.nonce,
        &info.balance,
        &storage_root(storage),
        &info.code_hash,
    ];

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
        .map(|(key, value)| {
            let data = alloy_rlp::encode(value.present_value);
            (
                Nibbles::unpack(alloy_sol_types::private::keccak256(key.to_be_bytes::<32>())),
                data,
            )
        })
        .collect::<Vec<_>>();
    storage.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

    storage
}

pub fn prove_storage(
    storage: &HashMap<U256, EvmStorageSlot>,
    keys: &[FixedBytes<32>],
) -> Vec<Vec<Bytes>> {
    let keys: Vec<_> = keys
        .iter()
        .map(|key| Nibbles::unpack(alloy_sol_types::private::keccak256(key)))
        .collect();

    let all_proof_nodes = build_proof(keys.clone(), trie_storage(storage));

    let mut proofs = Vec::new();
    for proof_key in keys {
        let matching_proof_nodes = all_proof_nodes
            .iter()
            .filter(|(path, _)| proof_key.starts_with(path))
            .map(|(_, node)| node.clone());
        proofs.push(matching_proof_nodes.collect());
    }

    proofs
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, b256, hex, U160};
    use serde_json::{from_str, from_value, Value};
    use std::fs;

    fn read_and_parse_json_file(file_path: &str) -> Value {
        let file_content = fs::read_to_string(file_path).expect("Failed to read the file");
        let json_value = from_str(&file_content).expect("Failed to parse JSON from file");
        json_value
    }

    fn build_state() -> EvmState {
        let json_value = read_and_parse_json_file("testdata/dumped_evm_state.json");
        let evm_state: EvmState = from_value(json_value).expect("Failed to parse EVM state");
        evm_state
    }

    #[test]
    fn test_address_to_nibbles() {
        let address = Address::from(U160::from(0x123456));
        assert_eq!(
            keccak256(address),
            b256!("f5c7e89ecc8b4dced430a51ceb6cac1af1067e6aef60306400853e88334e023c")
        );
        let nibbles = address_to_nibbles(address);
        assert_eq!(
            nibbles.as_slice(),
            hex!("0f050c070e08090e0c0c080b040d0c0e0d0403000a05010c0e0b060c0a0c010a0f010006070e060a0e0f06000300060400000805030e08080303040e0002030c")
        );
    }

    #[test]
    fn test_account_proof() {
        let evm_state = build_state();
        let address = address!();
        let proofs = account_proof(address, &evm_state);
        assert_eq!(proofs.len(), 1);
    }
}
