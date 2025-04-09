use std::convert::Infallible;

use crate::{sealed_with_test_mock, Digest};

pub type Result = std::result::Result<(), Infallible>;
sealed_with_test_mock!(IVerifier (proof: &Proof, vk: VerifyingKey) -> Result);

pub enum Proof {
    Guest(Vec<u8>),
    #[cfg(feature = "sp1-host")]
    Host(sp1_sdk::SP1ProofWithPublicValues),
}

pub enum VerifyingKey {
    Guest(Vec<u8>),
    #[cfg(feature = "sp1-host")]
    Host(sp1_sdk::SP1VerifyingKey),
}

pub struct GuestVerifier;

impl seal::Sealed for GuestVerifier {}
impl IVerifier for GuestVerifier {
    fn verify(&self, proof: &Proof, vk: VerifyingKey) -> Result {
        Ok(())
    }
}

#[cfg(feature = "sp1-host")]
pub struct HostVerifier;

#[cfg(feature = "sp1-host")]
impl seal::Sealed for HostVerifier {}

#[cfg(feature = "sp1-host")]
impl IVerifier for HostVerifier {
    fn verify(&self, proof: &Proof, vk: VerifyingKey) -> Result {
        Ok(())
    }
}
