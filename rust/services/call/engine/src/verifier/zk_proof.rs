use auto_impl::auto_impl;
use risc0_zkp::verify::VerificationError;
use risc0_zkvm::{guest, sha::Digest, Receipt};
use static_assertions::assert_obj_safe;

#[auto_impl(Fn)]
pub trait ZkpVerifier: Send + Sync + 'static {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError>;
}

assert_obj_safe!(ZkpVerifier);

pub struct GuestVerifier;

impl ZkpVerifier for GuestVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError> {
        guest::env::verify(elf_id, receipt.journal.as_ref()).expect("infallible");
        Ok(())
    }
}

pub struct HostVerifier;

impl ZkpVerifier for HostVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result<(), VerificationError> {
        receipt.verify(elf_id)
    }
}
