use async_trait::async_trait;
use derive_new::new;

use super::{sealing::sealed_with_test_mock, time_travel};
use crate::evm::{env::location::ExecutionLocation, input::MultiEvmInput};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Time travel error: {0}")]
    TimeTravel(#[from] super::time_travel::Error),
}

pub type Result = std::result::Result<(), Error>;
sealed_with_test_mock!(async IVerifier (input: &MultiEvmInput, start_execution_location: ExecutionLocation) -> Result);

#[derive(new)]
pub struct Verifier<TT: time_travel::IVerifier> {
    time_travel: TT,
}

impl<TT: time_travel::IVerifier> seal::Sealed for Verifier<TT> {}
#[async_trait]
impl<TT: time_travel::IVerifier> IVerifier for Verifier<TT> {
    async fn verify(
        &self,
        input: &MultiEvmInput,
        _start_execution_location: ExecutionLocation,
    ) -> Result {
        input.assert_coherency();
        for (chain_id, blocks) in input.blocks_by_chain() {
            self.time_travel.verify(chain_id, blocks).await?;
        }
        Ok(())
    }
}
