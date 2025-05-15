use std::sync::{Arc, RwLock};

use alloy_primitives::{Address, B256, BlockNumber, Bytes, U256};
use block_header::EvmBlockHeader;
use mpt::{KeccakMerkleTrie as MerkleTrie, ParseNodeError};
use provider::{BlockingProvider, EIP1186Proof};
use revm::{
    DatabaseRef,
    primitives::{AccountInfo, Bytecode, HashMap, HashSet},
};
use thiserror::Error;

use crate::provider::ProviderDb;

#[derive(Default, Debug)]
struct State {
    accounts: HashMap<Address, HashSet<U256>>,
    contracts: HashMap<B256, Bytes>,
    block_hash_numbers: HashSet<u64>,
}

#[derive(Debug)]
pub struct ProofDb {
    db: ProviderDb,
    state: RwLock<State>,
}

#[allow(clippy::expect_used)]
impl DatabaseRef for ProofDb {
    type Error = crate::provider::ProviderDbError;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        let basic = self.db.basic_ref(address)?;
        let mut state = self.state.write().expect("poisoned lock");
        state.accounts.entry(address).or_default();
        Ok(basic)
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        let code = self.db.code_by_hash_ref(code_hash)?;
        let mut state = self.state.write().expect("poisoned lock");
        state.contracts.insert(code_hash, code.original_bytes());
        Ok(code)
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let storage = self.db.storage_ref(address, index)?;
        let mut state = self.state.write().expect("poisoned lock");
        state.accounts.entry(address).or_default().insert(index);
        Ok(storage)
    }

    fn block_hash_ref(&self, number: u64) -> Result<B256, Self::Error> {
        let block_hash = self.db.block_hash_ref(number)?;
        let mut state = self.state.write().expect("poisoned lock");
        state.block_hash_numbers.insert(number);
        Ok(block_hash)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid state proof: {0}")]
    InvalidStateProof(ParseNodeError),
    #[error("Invalid storage proof: {0}")]
    InvalidStorageProof(ParseNodeError),
    #[error("Provider error: {0}")]
    Provider(#[from] provider::Error),
    #[error("Block not found: {0}")]
    BlockNotFound(BlockNumber),
}

#[allow(clippy::expect_used)]
impl ProofDb {
    pub fn new(provider: Arc<dyn BlockingProvider>, block_number: u64) -> Self {
        let state = RwLock::new(State::default());
        Self {
            state,
            db: ProviderDb::new(provider, block_number),
        }
    }

    pub fn contracts(&self) -> Vec<Bytes> {
        let state = self.state.read().expect("poisoned lock");
        state.contracts.values().cloned().collect()
    }

    pub fn fetch_ancestors(&self) -> Result<Vec<Box<dyn EvmBlockHeader>>, Error> {
        let state = self.state.read().expect("poisoned lock");
        let provider = &self.db.provider;
        let mut ancestors = Vec::new();
        if let Some(block_hash_min_number) = state.block_hash_numbers.iter().min() {
            for number in (*block_hash_min_number..self.db.block_number).rev() {
                let header = provider
                    .get_block_header(number.into())?
                    .ok_or(Error::BlockNotFound(number))?;
                ancestors.push(header);
            }
        }
        Ok(ancestors)
    }

    pub fn prepare_state_storage_tries(&self) -> Result<(MerkleTrie, Vec<MerkleTrie>), Error> {
        let proofs = self.fetch_proofs()?;
        let state_trie = Self::state_trie(&proofs)?;
        let storage_tries = Self::storage_tries(&proofs)?;
        Ok((state_trie, storage_tries))
    }

    fn fetch_proofs(&self) -> Result<Vec<EIP1186Proof>, Error> {
        let state = self.state.read().expect("poisoned lock");
        let mut proofs = Vec::new();
        for (address, storage_keys) in &state.accounts {
            let proof = self.db.provider.get_proof(
                *address,
                storage_keys.iter().map(|v| B256::from(*v)).collect(),
                self.db.block_number,
            )?;
            proofs.push(proof);
        }
        Ok(proofs)
    }

    pub fn state_trie(proofs: &[EIP1186Proof]) -> Result<MerkleTrie, Error> {
        let state_nodes = proofs.iter().flat_map(|p| p.account_proof.iter());
        let state_trie =
            MerkleTrie::from_rlp_nodes(state_nodes).map_err(Error::InvalidStateProof)?;
        Ok(state_trie)
    }

    fn storage_tries(proofs: &[EIP1186Proof]) -> Result<Vec<MerkleTrie>, Error> {
        proofs
            .iter()
            .filter(|proof| !(proof.storage_proof.is_empty() || proof.storage_hash.is_zero()))
            .map(|proof| {
                let storage_nodes = proof.storage_proof.iter().flat_map(|p| p.proof.iter());
                MerkleTrie::from_rlp_nodes(storage_nodes).map_err(Error::InvalidStorageProof)
            })
            .collect()
    }
}

pub use self::Error as ProofDbError;
