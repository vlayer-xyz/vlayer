use crate::provider::Provider;
use alloy_primitives::{Address, Sealable, B256, U256};
use revm::{
    primitives::{AccountInfo, Bytecode, HashMap, KECCAK_EMPTY},
    Database,
};
use std::fmt::Debug;
use std::rc::Rc;
use thiserror::Error;

/// Error type for the [ProviderDb].
#[derive(Error, Debug)]
pub enum ProviderDbError<E: std::error::Error> {
    #[error("provider error")]
    Provider(#[from] E),
    #[error("invalid block number: {0}")]
    InvalidBlockNumber(u64),
    #[error("hash missing for block: {0}")]
    BlockHashMissing(U256),
}

/// A revm [Database] backed by a [Provider].
pub struct ProviderDb<P> {
    pub provider: Rc<P>,
    pub block_number: u64,

    /// Cache for code hashes to contract addresses.
    code_hashes: HashMap<B256, Address>,
}

impl<P: Provider> ProviderDb<P> {
    /// Creates a new [ProviderDb] with the given provider and block number.
    pub fn new(provider: Rc<P>, block_number: u64) -> Self {
        Self {
            provider,
            block_number,
            code_hashes: HashMap::new(),
        }
    }
}

impl<P: Provider> Database for ProviderDb<P> {
    type Error = ProviderDbError<P::Error>;

    fn basic(&mut self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        // use `eth_getProof` to get all the account info with a single call
        let proof = self
            .provider
            .get_proof(address, vec![], self.block_number)?;
        // for non-existent accounts, the code hash is zero
        // see https://github.com/ethereum/go-ethereum/issues/28441
        if proof.code_hash == B256::ZERO {
            return Ok(None);
        }
        // cache the code hash to address mapping, so we can later retrieve the code
        self.code_hashes
            .insert(proof.code_hash.0.into(), proof.address);

        Ok(Some(AccountInfo {
            nonce: proof.nonce,
            balance: proof.balance,
            code_hash: proof.code_hash,
            code: None,
        }))
    }

    fn code_by_hash(&mut self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        // avoid querying the RPC if the code hash is empty
        if code_hash == KECCAK_EMPTY {
            return Ok(Bytecode::new());
        }

        // this works because we always call `basic_ref` first
        let contract_address = *self
            .code_hashes
            .get(&code_hash)
            .expect("`basic` must be called first for the corresponding account");
        let code = self
            .provider
            .get_code(contract_address, self.block_number)
            .map_err(ProviderDbError::Provider)?;

        Ok(Bytecode::new_raw(code.0.into()))
    }

    fn storage(&mut self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let storage = self
            .provider
            .get_storage_at(address, index.into(), self.block_number)
            .map_err(ProviderDbError::Provider)?;

        Ok(storage)
    }

    fn block_hash(&mut self, block_number: u64) -> Result<B256, Self::Error> {
        let header = self
            .provider
            .get_block_header(block_number)?
            .ok_or(ProviderDbError::InvalidBlockNumber(block_number))?;

        Ok(header.hash_slow())
    }
}
