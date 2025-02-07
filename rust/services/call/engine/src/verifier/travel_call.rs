use std::marker::PhantomData;

use async_trait::async_trait;
use call_common::{ExecutionLocation, RevmDB};
use derive_new::new;
use tracing::info;

use super::{teleport, time_travel};
use crate::evm::env::cached::CachedEvmEnv;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Time travel error: {0}")]
    TimeTravel(#[from] super::time_travel::Error),
    #[error("Teleport error: {0}")]
    Teleport(#[from] super::teleport::Error),
}

pub type Result = std::result::Result<(), Error>;
mod seal {
    use call_common::RevmDB;

    pub trait Sealed<D: RevmDB> {}
}

#[cfg(any(test, feature = "testing"))]
impl<F, D> seal::Sealed<D> for F
where
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result + Send + Sync,
    D: RevmDB,
{
}

#[async_trait]
pub trait IVerifier<D: RevmDB>: seal::Sealed<D> + Send + Sync {
    async fn verify(
        &self,
        input: &CachedEvmEnv<D>,
        start_execution_location: ExecutionLocation,
    ) -> Result;
}

#[cfg(any(test, feature = "testing"))]
#[async_trait]
impl<F, D> IVerifier<D> for F
where
    D: RevmDB,
    F: Fn(&CachedEvmEnv<D>, ExecutionLocation) -> Result + Send + Sync,
{
    async fn verify(
        &self,
        input: &CachedEvmEnv<D>,
        start_execution_location: ExecutionLocation,
    ) -> Result {
        self(input, start_execution_location)
    }
}

#[derive(new)]
pub struct Verifier<D, TT, TP>
where
    TT: time_travel::IVerifier,
    TP: teleport::IVerifier<D>,
    D: RevmDB,
{
    time_travel: TT,
    teleport: TP,
    _phantom_d: PhantomData<D>,
}

impl<TT: time_travel::IVerifier, TP: teleport::IVerifier<D>, D: RevmDB> seal::Sealed<D>
    for Verifier<D, TT, TP>
{
}

#[async_trait]
impl<TT: time_travel::IVerifier, TP: teleport::IVerifier<D>, D: RevmDB> IVerifier<D>
    for Verifier<D, TT, TP>
{
    async fn verify(
        &self,
        input: &CachedEvmEnv<D>,
        start_execution_location: ExecutionLocation,
    ) -> Result {
        info!("Verifying travel call");
        self.teleport
            .verify(input, start_execution_location)
            .await?;
        for (chain_id, blocks) in input.blocks_by_chain() {
            self.time_travel.verify(chain_id, blocks).await?;
        }
        Ok(())
    }
}
