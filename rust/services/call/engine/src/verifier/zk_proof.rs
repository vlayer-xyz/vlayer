pub use risc0_zkp::verify::VerificationError as Error;
use risc0_zkvm::{guest, sha::Digest, Receipt};
use static_assertions::assert_obj_safe;

pub type Result = std::result::Result<(), Error>;

mod seal {
    // This trait prevents adding new implementations of ZkpVerifier
    pub trait Sealed {}

    // Useful to mock verifier in tests
    #[cfg(test)]
    impl<F: Fn(&super::Receipt, super::Digest) -> super::Result> Sealed for F {}
}

#[cfg_attr(test, auto_impl::auto_impl(Fn))]
pub trait Verifier: seal::Sealed + Send + Sync + 'static {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result;
}

assert_obj_safe!(Verifier);

pub struct GuestVerifier;

impl seal::Sealed for GuestVerifier {}
impl Verifier for GuestVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result {
        guest::env::verify(elf_id, receipt.journal.as_ref()).expect("infallible");
        Ok(())
    }
}

pub struct HostVerifier;

impl seal::Sealed for HostVerifier {}
impl Verifier for HostVerifier {
    fn verify(&self, receipt: &Receipt, elf_id: Digest) -> Result {
        receipt.verify(elf_id)
    }
}
