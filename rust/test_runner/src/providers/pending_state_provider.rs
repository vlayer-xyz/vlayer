use block_header::{EvmBlockHeader, ForgeBlockHeader, Hashable};
use call_db::ProofDb;
use ethers_core::types::BlockNumber as BlockTag;
use forge::revm::primitives::{
    Account, Address, B256, Bytes, EvmState, U256,
    alloy_primitives::{BlockNumber, ChainId, StorageKey, StorageValue, TxNumber},
};
use provider::{BlockingProvider, EIP1186Proof, ProviderFactory, Result};

use crate::proof::{account_proof, prove_storage, storage_root};

#[derive(Debug)]
pub struct PendingStateProvider {
    state: EvmState,
    block_number: u64,
}

impl PendingStateProvider {
    fn account(&self, address: Address) -> Account {
        self.state.get(&address).cloned().unwrap_or_default()
    }

    fn all_account_proofs(&self) -> Vec<EIP1186Proof> {
        self.state
            .iter()
            .map(|(address, account)| {
                self.get_proof(*address, account.storage.keys().map(|v| (*v).into()).collect(), 0)
                    .unwrap()
            })
            .collect()
    }

    fn get_state_root(&self) -> B256 {
        let proofs = self.all_account_proofs();
        let state_trie = ProofDb::state_trie(&proofs);
        state_trie.map_or(Default::default(), |trie| trie.hash_slow())
    }
}

impl BlockingProvider for PendingStateProvider {
    fn get_balance(&self, address: Address, _block: BlockNumber) -> Result<U256> {
        Ok(self.account(address).info.balance)
    }

    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>> {
        let block_number: u64 = match block {
            BlockTag::Number(n) => n.as_u64(),
            _ => self.block_number,
        };
        Ok(Some(Box::new(ForgeBlockHeader::new(block_number, self.get_state_root()))))
    }

    fn get_code(&self, address: Address, _block: BlockNumber) -> Result<Bytes> {
        Ok(self
            .account(address)
            .info
            .code
            .map_or(Bytes::default(), |code| code.original_bytes()))
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        _block: BlockNumber,
    ) -> Result<EIP1186Proof> {
        let account = self.account(address);

        let account_proof = EIP1186Proof {
            address,
            balance: account.info.balance,
            nonce: account.info.nonce,
            code_hash: account.info.code_hash,
            storage_hash: storage_root(&account.storage),
            account_proof: account_proof(address, &self.state),
            storage_proof: prove_storage(&account.storage, &storage_keys),
        };

        Ok(account_proof)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        _block: BlockNumber,
    ) -> Result<StorageValue> {
        let storage_value = self
            .account(address)
            .storage
            .get(&key.into())
            .map_or(StorageValue::default(), |value| value.present_value);
        Ok(storage_value)
    }

    fn get_transaction_count(&self, address: Address, _block: BlockNumber) -> Result<TxNumber> {
        Ok(self.account(address).info.nonce)
    }

    fn get_latest_block_number(&self) -> Result<BlockNumber> {
        Ok(self.block_number)
    }
}

#[derive(Debug)]
pub struct PendingStateProviderFactory {
    pub block_number: u64,
    pub state: EvmState,
}

impl ProviderFactory for PendingStateProviderFactory {
    fn create(&self, _chain_id: ChainId) -> provider::factory::Result<Box<dyn BlockingProvider>> {
        Ok(Box::new(PendingStateProvider {
            state: self.state.clone(),
            block_number: self.block_number,
        }))
    }
}

#[cfg(test)]
mod test {
    use std::fs;

    use forge::revm::primitives::{address, b256};
    use mpt::KeccakMerkleTrie as MerkleTrie;
    use serde_json::{Value, from_str, from_value};

    use super::*;

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
    fn storage_proof_is_correctly_created_and_checked_by_mpt() {
        let state = build_state();
        let address = address!("5615deb798bb3e4dfa0139dfa1b3d433cc23b72f");
        let storage_keys =
            vec![b256!("db302bf24b1ad5f23949da8e6b05747dc699499a995361a7bf40ec7204696d6f")];

        let provider = PendingStateProvider {
            state,
            block_number: 0,
        };
        let proof = provider.get_proof(address, storage_keys, 0).unwrap();
        MerkleTrie::from_rlp_nodes(proof.storage_proof.iter().flat_map(|x| x.proof.clone()))
            .expect("Invalid proof");
    }
}
