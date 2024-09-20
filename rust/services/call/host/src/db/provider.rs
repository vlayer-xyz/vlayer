use crate::provider::BlockingProvider;
use alloy_primitives::{Address, B256, U256};
use revm::{
    primitives::{AccountInfo, Bytecode, HashMap, KECCAK_EMPTY},
    DatabaseRef,
};
use std::rc::Rc;
use std::{cell::RefCell, fmt::Debug};
use thiserror::Error;

/// Error type for the [ProviderDb].
#[derive(Error, Debug)]
pub enum ProviderDbError<E: std::error::Error> {
    #[error("provider error")]
    Provider(#[from] E),
    #[error("invalid block number: {0}")]
    InvalidBlockNumber(U256),
    #[error("hash missing for block: {0}")]
    BlockHashMissing(U256),
}

/// A revm [Database] backed by a [Provider].
pub(crate) struct ProviderDb<P> {
    pub(crate) provider: Rc<P>,
    pub(crate) block_number: u64,

    /// Cache for code hashes to contract addresses.
    code_hashes: RefCell<HashMap<B256, Address>>,
}

impl<P: BlockingProvider> ProviderDb<P> {
    /// Creates a new [ProviderDb] with the given provider and block number.
    pub(crate) fn new(provider: Rc<P>, block_number: u64) -> Self {
        Self {
            provider,
            block_number,
            code_hashes: RefCell::new(HashMap::new()),
        }
    }
}

impl<P: BlockingProvider> DatabaseRef for ProviderDb<P> {
    type Error = ProviderDbError<P::Error>;

    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
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
            .borrow_mut()
            .insert(proof.code_hash.0.into(), proof.address);

        Ok(Some(AccountInfo {
            nonce: proof.nonce,
            balance: proof.balance,
            code_hash: proof.code_hash,
            code: None,
        }))
    }

    fn code_by_hash_ref(&self, code_hash: B256) -> Result<Bytecode, Self::Error> {
        // avoid querying the RPC if the code hash is empty
        if code_hash == KECCAK_EMPTY {
            return Ok(Bytecode::new());
        }

        // this works because we always call `basic_ref` first
        let contract_address = *self
            .code_hashes
            .borrow()
            .get(&code_hash)
            .expect("`basic` must be called first for the corresponding account");
        let code = self
            .provider
            .get_code(contract_address, self.block_number)
            .map_err(ProviderDbError::Provider)?;

        Ok(Bytecode::new_raw(code.0.into()))
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        let storage = self
            .provider
            .get_storage_at(address, index.into(), self.block_number)
            .map_err(ProviderDbError::Provider)?;

        Ok(storage)
    }

    fn block_hash_ref(&self, number: U256) -> Result<B256, Self::Error> {
        let block_number: u64 = number
            .try_into()
            .map_err(|_| ProviderDbError::InvalidBlockNumber(number))?;
        let header = self
            .provider
            .get_block_header(block_number.into())?
            .ok_or(ProviderDbError::InvalidBlockNumber(number))?;

        Ok(header.hash_slow())
    }
}
