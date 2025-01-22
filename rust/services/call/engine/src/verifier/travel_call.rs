use async_trait::async_trait;
use derive_new::new;
use static_assertions::assert_obj_safe;

use super::{impl_verifier_for_fn, sealed_trait, time_travel, verifier_trait};
use crate::evm::input::MultiEvmInput;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Time travel error: {0}")]
    TimeTravel(#[from] super::time_travel::Error),
}

pub type Result = std::result::Result<(), Error>;

sealed_trait!(&super::MultiEvmInput);
verifier_trait!(async (input: &MultiEvmInput) -> Result);
impl_verifier_for_fn!(async (input: &MultiEvmInput) -> Result);

#[derive(new)]
pub struct Verifier<TT: time_travel::IVerifier> {
    time_travel: TT,
}

impl<TT: time_travel::IVerifier> Verifier<TT> {
    pub fn into_time_travel_verifier(self) -> TT {
        self.time_travel
    }
}

impl<TT: time_travel::IVerifier> seal::Sealed for Verifier<TT> {}
#[async_trait]
impl<TT: time_travel::IVerifier> IVerifier for Verifier<TT> {
    async fn verify(&self, input: &MultiEvmInput) -> Result {
        for (chain_id, blocks) in input.blocks_by_chain() {
            self.time_travel.verify(chain_id, blocks).await?;
        }
        Ok(())
    }
}
