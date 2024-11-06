#[cfg(test)]
use auto_impl::auto_impl;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{guest, sha::Digest, Receipt};
use static_assertions::assert_obj_safe;

mod seal {
    // This trait prevents adding new implementations of ZkpVerifier
    pub trait Sealed {}

    // Useful to mock verifier in tests
    #[cfg(test)]
    impl<F: Fn(&super::Receipt, super::Digest) -> Result<(), super::VerificationError>> Sealed for F {}
}

#[cfg_attr(test, auto_impl(Fn))]
pub trait ZkpVerifier: seal::Sealed + Send + Sync + 'static {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError>;
}

assert_obj_safe!(ZkpVerifier);

pub struct GuestVerifier;

impl seal::Sealed for GuestVerifier {}
impl ZkpVerifier for GuestVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError> {
        guest::env::verify(elf_id, receipt.journal.as_ref()).expect("infallible");
        Ok(())
    }
}

pub struct HostVerifier;

impl seal::Sealed for HostVerifier {}
impl ZkpVerifier for HostVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError> {
        receipt.verify(elf_id)
    }
}
