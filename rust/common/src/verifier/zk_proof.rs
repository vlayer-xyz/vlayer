pub use risc0_zkp::verify::VerificationError as Error;
use risc0_zkvm::{Receipt, guest, sha::Digest};

use super::sealing::sealed_with_test_mock;

pub type Result = std::result::Result<(), Error>;
sealed_with_test_mock!(IVerifier (receipt: &Receipt, elf_id: Digest) -> Result);

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
