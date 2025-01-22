pub use risc0_zkp::verify::VerificationError as Error;
use risc0_zkvm::{guest, sha::Digest, Receipt};
use static_assertions::assert_obj_safe;

use super::{sealed_trait, verifier_trait};

pub type Result = std::result::Result<(), Error>;

sealed_trait!(&super::Receipt, super::Digest);
verifier_trait!((receipt: &Receipt, elf_id: Digest) -> Result);

#[cfg(any(test, feature = "testing"))]
impl<F: Fn(&Receipt, Digest) -> Result + Send + Sync> IVerifier for F {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result {
        self(receipt, elf_id)
    }
}

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
