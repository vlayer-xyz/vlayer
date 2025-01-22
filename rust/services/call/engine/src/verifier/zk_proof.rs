pub use risc0_zkp::verify::VerificationError as Error;
use risc0_zkvm::{guest, sha::Digest, Receipt};
use static_assertions::assert_obj_safe;

use super::define_sealed_trait;

pub type Result = std::result::Result<(), Error>;

define_sealed_trait!(&super::Receipt, super::Digest);

#[cfg_attr(test, auto_impl::auto_impl(Fn))]
pub trait IVerifier: seal::Sealed + Send + Sync + 'static {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result;
}

assert_obj_safe!(IVerifier);

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
