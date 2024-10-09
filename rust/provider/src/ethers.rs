use core::future::Future;

use alloy_primitives::{BlockNumber, B256, U256};
use block_header::{EthBlockHeader, EvmBlockHeader};
use ethers_core::types::{Block, BlockNumber as BlockTag};
use ethers_providers::{Middleware, MiddlewareError};
use thiserror::Error;
use tokio::runtime::Handle;

use super::{BlockingProvider, EIP1186Proof};

/// An error that can occur when interacting with the provider.
#[derive(Error, Debug)]
pub enum EthersProviderError<M: MiddlewareError> {
    #[error("middleware error: {0}")]
    MiddlewareError(#[from] M),
    #[error("block conversion error: {0}")]
    BlockConversionError(String),
}

/// A provider that fetches data from an Ethereum node using the ethers crate.
pub struct EthersProvider<M: Middleware> {
    client: M,
}

impl<M: Middleware> EthersProvider<M> {
    pub(crate) fn new(client: M) -> Self {
        Self { client }
    }
}

// Blocks current runtime to execute the future. Panics if called outside of the runtime
fn block_on<F: Future>(f: F) -> F::Output {
    let handle = Handle::try_current().expect("no tokio runtime");
    tokio::task::block_in_place(|| handle.block_on(f))
}

impl<M: Middleware> BlockingProvider for EthersProvider<M>
where
    M::Error: 'static,
{
    type Error = EthersProviderError<M::Error>;

    fn get_block_header(
        &self,
        block: BlockTag,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        let block = block_on(self.client.get_block(block))?;
        match block {
            Some(block) => {
                let eth_block_header = to_eth_block_header(block)
                    .map_err(EthersProviderError::BlockConversionError)?;
                Ok(Some(Box::new(eth_block_header) as Box<dyn EvmBlockHeader>))
            }
            None => Ok(None),
        }
    }

    fn get_transaction_count(
        &self,
        address: alloy_primitives::Address,
        block: BlockNumber,
    ) -> Result<alloy_primitives::TxNumber, Self::Error> {
        let address = to_ethers_h160(address);
        let count = block_on(
            self.client
                .get_transaction_count(address, Some(block.into())),
        )
        .map(from_ethers_u256)?;
        Ok(count.to())
    }

    fn get_balance(
        &self,
        address: alloy_primitives::Address,
        block: BlockNumber,
    ) -> Result<alloy_primitives::U256, Self::Error> {
        let address = to_ethers_h160(address);
        Ok(from_ethers_u256(block_on(
            self.client.get_balance(address, Some(block.into())),
        )?))
    }

    fn get_code(
        &self,
        address: alloy_primitives::Address,
        block: BlockNumber,
    ) -> Result<alloy_primitives::Bytes, Self::Error> {
        let address = to_ethers_h160(address);
        Ok(from_ethers_bytes(block_on(self.client.get_code(address, Some(block.into())))?))
    }

    fn get_storage_at(
        &self,
        address: alloy_primitives::Address,
        key: alloy_primitives::StorageKey,
        block: BlockNumber,
    ) -> Result<alloy_primitives::StorageValue, Self::Error> {
        let address = to_ethers_h160(address);
        let key = to_ethers_h256(key);
        let value = block_on(self.client.get_storage_at(address, key, Some(block.into())))?;

        Ok(from_ethers_h256(value).into())
    }

    fn get_proof(
        &self,
        address: alloy_primitives::Address,
        storage_keys: Vec<alloy_primitives::StorageKey>,
        block: BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        let address = to_ethers_h160(address);
        let storage_keys = storage_keys.into_iter().map(to_ethers_h256).collect();
        let proof = block_on(
            self.client
                .get_proof(address, storage_keys, Some(block.into())),
        )?;

        Ok(EIP1186Proof {
            address: address.0.into(),
            balance: from_ethers_u256(proof.balance),
            code_hash: from_ethers_h256(proof.code_hash),
            nonce: proof.nonce.as_u64(),
            storage_hash: from_ethers_h256(proof.storage_hash),
            account_proof: proof
                .account_proof
                .into_iter()
                .map(from_ethers_bytes)
                .collect(),
            storage_proof: proof.storage_proof.into_iter().map(Into::into).collect(),
        })
    }
}

pub fn to_eth_block_header<T>(block: Block<T>) -> Result<EthBlockHeader, String> {
    Ok(EthBlockHeader {
        parent_hash: from_ethers_h256(block.parent_hash),
        ommers_hash: from_ethers_h256(block.uncles_hash),
        beneficiary: block.author.ok_or("author missing")?.0.into(),
        state_root: from_ethers_h256(block.state_root),
        transactions_root: from_ethers_h256(block.transactions_root),
        receipts_root: from_ethers_h256(block.receipts_root),
        logs_bloom: alloy_primitives::Bloom::from_slice(
            block.logs_bloom.ok_or("bloom missing")?.as_bytes(),
        ),
        difficulty: from_ethers_u256(block.difficulty),
        number: block.number.ok_or("number is missing")?.as_u64(),
        gas_limit: block
            .gas_limit
            .try_into()
            .map_err(|_| "invalid gas limit")?,
        gas_used: block.gas_used.try_into().map_err(|_| "invalid gas used")?,
        timestamp: block
            .timestamp
            .try_into()
            .map_err(|_| "invalid timestamp")?,
        extra_data: block.extra_data.0.into(),
        mix_hash: from_ethers_h256(block.mix_hash.ok_or("mix_hash is missing")?),
        nonce: block.nonce.ok_or("nonce is missing")?.0.into(),
        base_fee_per_gas: from_ethers_u256(
            block
                .base_fee_per_gas
                .ok_or("base_fee_per_gas is missing")?,
        ),
        withdrawals_root: block.withdrawals_root.map(from_ethers_h256),
        blob_gas_used: block.blob_gas_used.map(TryInto::try_into).transpose()?,
        excess_blob_gas: block.excess_blob_gas.map(TryInto::try_into).transpose()?,
        parent_beacon_block_root: block.parent_beacon_block_root.map(from_ethers_h256),
    })
}

pub(crate) fn from_ethers_bytes(v: ethers_core::types::Bytes) -> alloy_primitives::Bytes {
    v.0.into()
}

pub(crate) fn to_ethers_h256(v: alloy_primitives::B256) -> ethers_core::types::H256 {
    v.0.into()
}

pub(crate) fn from_ethers_h256(v: ethers_core::types::H256) -> B256 {
    v.0.into()
}

pub(crate) fn from_ethers_u256(v: ethers_core::types::U256) -> U256 {
    alloy_primitives::U256::from_limbs(v.0)
}

pub(crate) fn to_ethers_h160(v: alloy_primitives::Address) -> ethers_core::types::H160 {
    v.into_array().into()
}
