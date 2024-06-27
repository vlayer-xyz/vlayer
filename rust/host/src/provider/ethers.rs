use super::{EIP1186Proof, Provider, StorageProof};
use ethers_providers::{Middleware, MiddlewareError};
use thiserror::Error;
use tokio::runtime::{Handle, Runtime};
use vlayer_engine::ethereum::{
    from_ethers_bytes, from_ethers_h256, from_ethers_u256, to_ethers_h160, to_ethers_h256,
    EthBlockHeader,
};

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

impl<M: Middleware> Provider for EthersProvider<M>
where
    M::Error: 'static,
{
    type Error = EthersProviderError<M::Error>;
    type Header = EthBlockHeader;

    fn get_block_header(
        &self,
        block: alloy_primitives::BlockNumber,
    ) -> Result<Option<Self::Header>, Self::Error> {
        let block = self.block_on(self.client.get_block(block))?;
        match block {
            Some(block) => Ok(Some(
                block
                    .try_into()
                    .map_err(EthersProviderError::BlockConversionError)?,
            )),
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
            storage_proof: proof
                .storage_proof
                .into_iter()
                .map(|p| StorageProof {
                    key: from_ethers_u256(p.key).to_be_bytes().into(),
                    proof: p.proof.into_iter().map(from_ethers_bytes).collect(),
                    value: from_ethers_u256(p.value),
                })
                .collect(),
        })
    }
}
