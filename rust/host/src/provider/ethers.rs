use super::{BlockingProvider, EIP1186Proof};
use alloy_primitives::{B256, U256};
use ethers_core::types::Block;
use ethers_providers::{Middleware, MiddlewareError};
use thiserror::Error;
use tokio::runtime::{Handle, Runtime};
use vlayer_engine::block_header::{eth::EthBlockHeader, EvmBlockHeader};

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
    runtime_handle: (Handle, Option<Runtime>),
}

impl<M: Middleware> EthersProvider<M> {
    pub fn new(client: M) -> Self {
        // if we are not in a tokio runtime, we need to create a new handle
        let runtime_handle = match Handle::try_current() {
            Ok(handle) => (handle, None),
            Err(_) => {
                #[allow(clippy::unwrap_used)]
                let runtime = Runtime::new().unwrap();
                (runtime.handle().clone(), Some(runtime))
            }
        };

        Self {
            client,
            runtime_handle,
        }
    }

    /// Fetches the current block number.
    pub fn get_block_number(&self) -> Result<alloy_primitives::BlockNumber, M::Error> {
        Ok(self.block_on(self.client.get_block_number())?.as_u64())
    }

    /// internal utility function to call tokio feature and wait for output
    fn block_on<F: core::future::Future>(&self, f: F) -> F::Output {
        self.runtime_handle.0.block_on(f)
    }
}

impl<M: Middleware> BlockingProvider for EthersProvider<M>
where
    M::Error: 'static,
{
    type Error = EthersProviderError<M::Error>;

    fn get_block_header(
        &self,
        block: ethers_core::types::BlockNumber,
    ) -> Result<Option<Box<dyn EvmBlockHeader>>, Self::Error> {
        let block = self.block_on(self.client.get_block(block))?;
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
        block: alloy_primitives::BlockNumber,
    ) -> Result<alloy_primitives::TxNumber, Self::Error> {
        let address = to_ethers_h160(address);
        let count = self
            .block_on(
                self.client
                    .get_transaction_count(address, Some(block.into())),
            )
            .map(from_ethers_u256)?;
        Ok(count.to())
    }

    fn get_balance(
        &self,
        address: alloy_primitives::Address,
        block: alloy_primitives::BlockNumber,
    ) -> Result<alloy_primitives::U256, Self::Error> {
        let address = to_ethers_h160(address);
        Ok(from_ethers_u256(self.block_on(
            self.client.get_balance(address, Some(block.into())),
        )?))
    }

    fn get_code(
        &self,
        address: alloy_primitives::Address,
        block: alloy_primitives::BlockNumber,
    ) -> Result<alloy_primitives::Bytes, Self::Error> {
        let address = to_ethers_h160(address);
        Ok(from_ethers_bytes(self.block_on(
            self.client.get_code(address, Some(block.into())),
        )?))
    }

    fn get_storage_at(
        &self,
        address: alloy_primitives::Address,
        key: alloy_primitives::StorageKey,
        block: alloy_primitives::BlockNumber,
    ) -> Result<alloy_primitives::StorageValue, Self::Error> {
        let address = to_ethers_h160(address);
        let key = to_ethers_h256(key);
        let value = self.block_on(self.client.get_storage_at(address, key, Some(block.into())))?;

        Ok(from_ethers_h256(value).into())
    }

    fn get_proof(
        &self,
        address: alloy_primitives::Address,
        storage_keys: Vec<alloy_primitives::StorageKey>,
        block: alloy_primitives::BlockNumber,
    ) -> Result<EIP1186Proof, Self::Error> {
        let address = to_ethers_h160(address);
        let storage_keys = storage_keys.into_iter().map(to_ethers_h256).collect();
        let proof = self.block_on(self.client.get_proof(
            address,
            storage_keys,
            Some(block.into()),
        ))?;

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

fn to_eth_block_header<T>(block: Block<T>) -> Result<EthBlockHeader, String> {
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
        blob_gas_used: block.blob_gas_used.map(|x| x.try_into()).transpose()?,
        excess_blob_gas: block.excess_blob_gas.map(|x| x.try_into()).transpose()?,
        parent_beacon_block_root: block.parent_beacon_block_root.map(from_ethers_h256),
    })
}

pub fn from_ethers_bytes(v: ethers_core::types::Bytes) -> alloy_primitives::Bytes {
    v.0.into()
}

pub fn to_ethers_h256(v: alloy_primitives::B256) -> ethers_core::types::H256 {
    v.0.into()
}

pub fn from_ethers_h256(v: ethers_core::types::H256) -> B256 {
    v.0.into()
}

pub fn from_ethers_u256(v: ethers_core::types::U256) -> U256 {
    alloy_primitives::U256::from_limbs(v.0)
}

pub fn to_ethers_h160(v: alloy_primitives::Address) -> ethers_core::types::H160 {
    v.into_array().into()
}
