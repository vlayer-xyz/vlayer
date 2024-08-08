use std::borrow::Borrow;
use std::convert::Infallible;

use ethers_core::types::BlockNumber as BlockTag;
use forge::revm::primitives::alloy_primitives::{
    BlockNumber, ChainId, StorageKey, StorageValue, TxNumber,
};
use forge::revm::primitives::{Account, Address, Bytes, EvmState, B256, U256};
use host::db::proof::ProofDb;
use host::host::error::HostError;
use host::proof::{EIP1186Proof, StorageProof};
use host::provider::factory::ProviderFactory;
use host::provider::BlockingProvider;
use vlayer_engine::block_header::eth::EthBlockHeader;
use vlayer_engine::block_header::EvmBlockHeader;
use vlayer_engine::config::MAINNET_MERGE_BLOCK_NUMBER;

use crate::proof::{account_proof, prove_storage, storage_root};

pub struct PendingStateProvider {
    state: EvmState,
}

impl PendingStateProvider {
    fn account(&self, address: Address) -> Account {
        match self.state.get(&address) {
            Some(account) => account.clone(),
            None => Account::default(),
        }
    }

    fn proofs(&self) -> anyhow::Result<Vec<EIP1186Proof>> {
        let state = self.state.borrow();
        let mut proofs = Vec::new();
        for (address, account) in state {
            let proof = self.get_proof(
                *address,
                account.storage.keys().map(|v| B256::from(*v)).collect(),
                0,
            )?;
            proofs.push(proof);
        }
        Ok(proofs)
    }

    fn get_state_root(&self) -> anyhow::Result<B256> {
        let proofs = self.proofs()?;
        let state_trie = ProofDb::<PendingStateProvider>::state_trie(&proofs)?;
        Ok(state_trie.hash_slow())
    }
}

impl BlockingProvider for PendingStateProvider {
    type Error = Infallible;

    fn get_balance(&self, address: Address, _block: BlockNumber) -> Result<U256, Self::Error> {
        Ok(self.account(address).info.balance)
    }

    fn get_block_header(
        &self,
        _block: BlockTag,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        Ok(Some(Box::new(EthBlockHeader {
            number: MAINNET_MERGE_BLOCK_NUMBER,
            state_root: self.get_state_root().unwrap_or_default(),
            ..EthBlockHeader::default()
        })))
    }

    fn get_code(&self, address: Address, _block: BlockNumber) -> Result<Bytes, Self::Error> {
        Ok(self
            .account(address)
            .info
            .code
            .clone()
            .map_or(Bytes::default(), |code| code.original_bytes()))
    }

    fn get_proof(
        &self,
        address: Address,
        storage_keys: Vec<StorageKey>,
        _block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        let account = self.account(address);
        let storage_proofs = prove_storage(&account.storage, &storage_keys);

        let account_proof = EIP1186Proof {
            address,
            balance: account.info.balance,
            nonce: account.info.nonce,
            code_hash: account.info.code_hash,
            storage_hash: storage_root(&account.storage),
            account_proof: account_proof(address, &self.state),
            storage_proof: storage_keys
                .into_iter()
                .zip(storage_proofs)
                .map(|(key, proof)| {
                    let storage_key: U256 = key.into();
                    let value = account
                        .storage
                        .get(&storage_key)
                        .cloned()
                        .unwrap_or_default()
                        .present_value;
                    StorageProof { key, value, proof }
                })
                .collect(),
        };

        Ok(account_proof)
    }

    fn get_storage_at(
        &self,
        address: Address,
        key: StorageKey,
        _block: BlockNumber,
    ) -> Result<StorageValue, Self::Error> {
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
    ) -> Result<TxNumber, Self::Error> {
        Ok(self.account(address).info.nonce)
    }
}

pub struct PendingStateProviderFactory {
    pub state: EvmState,
}

impl ProviderFactory<PendingStateProvider> for PendingStateProviderFactory {
    fn create(&self, _chain_id: ChainId) -> Result<PendingStateProvider, HostError> {
        Ok(PendingStateProvider {
            state: self.state.clone(),
        })
    }
}
