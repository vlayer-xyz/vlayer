pub use risc0_zkp::verify::VerificationError as Error;
use risc0_zkvm::{guest, sha::Digest, Receipt};

use super::{
    impl_sealed_for_fn, impl_verifier_for_fn, sealed_trait, setup_verifier_mocking, verifier_trait,
};

pub type Result = std::result::Result<(), Error>;
setup_verifier_mocking!((receipt: &Receipt, elf_id: Digest) -> Result);

pub struct GuestVerifier;

impl seal::Sealed for GuestVerifier {}
impl IVerifier for GuestVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result {
        guest::env::verify(elf_id, receipt.journal.as_ref()).expect("infallible");
        Ok(())
    }
}

pub struct HostVerifier;

impl seal::Sealed for HostVerifier {}
impl IVerifier for HostVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result {
        receipt.verify(elf_id)
    }
}
