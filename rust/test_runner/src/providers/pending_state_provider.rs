use anyhow::Error;
use block_header::{EvmBlockHeader, ForgeBlockHeader};
use call_host::db::proof::ProofDb;
use ethers_core::types::BlockNumber as BlockTag;
use forge::revm::primitives::{
    alloy_primitives::{BlockNumber, ChainId, StorageKey, StorageValue, TxNumber},
    Account, Address, Bytes, EvmState, B256, U256,
};
use provider::{BlockingProvider, EIP1186Proof, ProviderFactory, ProviderFactoryError};

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
        let state_trie = ProofDb::<PendingStateProvider>::state_trie(&proofs);
        state_trie.map_or(Default::default(), |trie| trie.hash_slow())
    }
}

impl BlockingProvider for PendingStateProvider {
    fn get_balance(&self, address: Address, _block: BlockNumber) -> Result<U256, Error> {
        Ok(self.account(address).info.balance)
    }

    fn get_block_header(&self, block: BlockTag) -> Result<Option<Box<dyn EvmBlockHeader>>, Error> {
        let block_number: u64 = match block {
            BlockTag::Number(n) => n.as_u64(),
            _ => self.block_number,
        };
        Ok(Some(Box::new(ForgeBlockHeader::new(block_number, self.get_state_root()))))
    }

    fn get_code(&self, address: Address, _block: BlockNumber) -> Result<Bytes, Error> {
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
    ) -> Result<EIP1186Proof, Error> {
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
    ) -> Result<StorageValue, Error> {
        let storage_value = self
            .account(address)
            .storage
            .get(&key.into())
            .map_or(StorageValue::default(), |value| value.present_value);
        Ok(storage_value)
    }

    fn get_transaction_count(
        &self,
        address: Address,
        _block: BlockNumber,
    ) -> Result<TxNumber, Error> {
        Ok(self.account(address).info.nonce)
    }
}

#[derive(Debug)]
pub struct PendingStateProviderFactory {
    pub block_number: u64,
    pub state: EvmState,
}

impl ProviderFactory<PendingStateProvider> for PendingStateProviderFactory {
    fn create(&self, _chain_id: ChainId) -> Result<PendingStateProvider, ProviderFactoryError> {
        Ok(PendingStateProvider {
            state: self.state.clone(),
            block_number: self.block_number,
        })
    }
}
