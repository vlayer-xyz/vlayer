use async_trait::async_trait;
use derive_new::new;

use super::{sealing::sealed_with_test_mock, teleport, time_travel};
use crate::evm::{env::location::ExecutionLocation, input::MultiEvmInput};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Time travel error: {0}")]
    TimeTravel(#[from] super::time_travel::Error),
    #[error("Teleport error: {0}")]
    Teleport(#[from] super::teleport::Error),
}

pub type Result = std::result::Result<(), Error>;
sealed_with_test_mock!(async IVerifier (input: &MultiEvmInput, start_execution_location: ExecutionLocation) -> Result);

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
