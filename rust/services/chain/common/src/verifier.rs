use risc0_zkvm::{guest::env, sha::Digest};
use static_assertions::assert_obj_safe;

pub trait GuestVerifier: Send + Sync + 'static {
    /// Verify ZK proof from guest code. This entails that verifying the receipt of the currently
    /// executing guest code will also guarantee that the given proof is valid.
    /// Panic if proof is invalid.
    fn verify(&self, elf_id: Digest, output: &[u8]);
}

assert_obj_safe!(GuestVerifier);

pub struct Risc0Verifier;

impl GuestVerifier for Risc0Verifier {
    fn verify(&self, elf_id: Digest, proof: &[u8]) {
        env::verify(elf_id, proof).expect("infallible");
    }
}

pub struct MockVerifier {
    verification_ok: bool,
}

impl MockVerifier {
    pub const fn new(verification_ok: bool) -> Self {
        Self { verification_ok }
    }
}

impl GuestVerifier for MockVerifier {
    fn verify(&self, _elf_id: Digest, _proof: &[u8]) {
        assert!(self.verification_ok, "proof verification failed")
    }
}
