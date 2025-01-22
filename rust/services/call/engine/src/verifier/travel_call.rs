use async_trait::async_trait;
use derive_new::new;
use static_assertions::assert_obj_safe;

use super::time_travel;
use crate::evm::input::MultiEvmInput;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Time travel error: {0}")]
    TimeTravel(#[from] super::time_travel::Error),
}

pub type Result = std::result::Result<(), Error>;

mod seal {

    // This trait prevents adding new implementations of Verifier
    pub trait Sealed {}

    // Useful to mock verifier in tests
    #[cfg(feature = "testing")]
    impl<F> Sealed for F where F: Fn(&super::MultiEvmInput) -> super::Result + Send + Sync {}
}

#[async_trait]
pub trait Verifier: seal::Sealed + Send + Sync {
    async fn verify(&self, input: &MultiEvmInput) -> Result;
}

assert_obj_safe!(Verifier);

// Useful to mock verifier in tests
// [auto_impl(Fn)] doesn't work with async_trait
#[cfg(feature = "testing")]
#[async_trait]
impl<F: Fn(&MultiEvmInput) -> Result + Send + Sync> Verifier for F {
    async fn verify(&self, input: &MultiEvmInput) -> Result {
        self(input)
    }
}

#[derive(new)]
pub struct ZkVerifier<TT: time_travel::Verifier> {
    time_travel: TT,
}

impl<TT: time_travel::Verifier> ZkVerifier<TT> {
    pub fn into_time_travel_verifier(self) -> TT {
        self.time_travel
    }
}

impl<TT: time_travel::Verifier> seal::Sealed for ZkVerifier<TT> {}
#[async_trait]
impl<TT: time_travel::Verifier> Verifier for ZkVerifier<TT> {
    async fn verify(&self, input: &MultiEvmInput) -> Result {
        for (chain_id, blocks) in input.blocks_by_chain() {
            self.time_travel.verify(chain_id, blocks).await?;
        }
        Ok(())
    }
}
