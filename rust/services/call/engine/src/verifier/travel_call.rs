use async_trait::async_trait;
use derive_new::new;

use super::{teleport, time_travel};
use crate::evm::{env::location::ExecutionLocation, input::MultiEvmInput};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Time travel error: {0}")]
    TimeTravel(#[from] super::time_travel::Error),
    #[error("Teleport error: {0}")]
    Teleport(#[from] super::teleport::Error),
}

pub type Result = std::result::Result<(), Error>;
crate::verifier::sealing::sealed_trait!();

#[cfg(any(test, feature = "testing"))]
impl<F> seal::Sealed for F where F: Fn(&MultiEvmInput, ExecutionLocation) -> Result + Send + Sync {}

#[async_trait]
pub trait IVerifier: seal::Sealed + Send + Sync {
    async fn verify(
        &self,
        input: &MultiEvmInput,
        start_execution_location: ExecutionLocation,
    ) -> Result;
}

#[cfg(any(test, feature = "testing"))]
#[async_trait::async_trait]
impl<F> IVerifier for F
where
    F: Fn(&MultiEvmInput, ExecutionLocation) -> Result + Send + Sync,
{
    async fn verify(
        &self,
        input: &MultiEvmInput,
        start_execution_location: ExecutionLocation,
    ) -> Result {
        self(input, start_execution_location)
    }
}

#[derive(new)]
pub struct Verifier<TT: time_travel::IVerifier, TP: teleport::IVerifier> {
    time_travel: TT,
    teleport: TP,
}

impl<TT: time_travel::IVerifier, TP: teleport::IVerifier> seal::Sealed for Verifier<TT, TP> {}
#[async_trait]
impl<TT: time_travel::IVerifier, TP: teleport::IVerifier> IVerifier for Verifier<TT, TP> {
    async fn verify(
        &self,
        input: &MultiEvmInput,
        start_execution_location: ExecutionLocation,
    ) -> Result {
        self.teleport
            .verify(input.blocks_by_chain(), start_execution_location)
            .await?;
        for (chain_id, blocks) in input.blocks_by_chain() {
            self.time_travel.verify(chain_id, blocks).await?;
        }
        input.assert_coherency();
        Ok(())
    }
}
