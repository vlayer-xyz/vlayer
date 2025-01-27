use alloy_primitives::ChainId;
use async_trait::async_trait;
use derive_new::new;
use revm::DatabaseRef;

use crate::evm::env::{cached::CachedEvmEnv, location::ExecutionLocation};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Unsupported chain id: {0}")]
    UnsupportedChainId(ChainId),
}

pub type Result = std::result::Result<(), Error>;
mod seal {
    pub trait Sealed<D> {}
}

#[cfg(any(test, feature = "testing"))]
impl<F, D> seal::Sealed<D> for F
where
    D: DatabaseRef,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result + Send + Sync,
{
}

#[async_trait]
pub trait IVerifier<D>: seal::Sealed<D> + Send + Sync
where
    D: DatabaseRef + Send + Sync,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result;
}

#[cfg(any(test, feature = "testing"))]
#[async_trait]
impl<F, D> IVerifier<D> for F
where
    D: DatabaseRef + Send + Sync,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result + Send + Sync,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        start_exec_location: ExecutionLocation,
    ) -> Result {
        self(evm_envs, start_exec_location)
    }
}

#[derive(new)]
pub struct Verifier {}

impl<D> seal::Sealed<D> for Verifier {}
#[async_trait]
impl<D> IVerifier<D> for Verifier
where
    D: DatabaseRef + Send + Sync,
{
    async fn verify(
        &self,
        evm_envs: &CachedEvmEnv<D>,
        _start_exec_location: ExecutionLocation,
    ) -> Result {
        let chains = evm_envs.blocks_by_chain().chain_ids();
        #[allow(clippy::match_same_arms)]
        match chains.as_ref() {
            [] => unreachable!(
                "Empty list of execution locations. At least start one should always be there"
            ),
            [_] => {
                Ok(()) // No teleportation
            }
            _ => {
                Ok(()) // TODO: Implement teleportation
            }
        }
    }
}
