use common::{Digest, GuestElf};
use host_utils::{ProofMode, ProofProvider};

pub struct Config {
    pub proof_mode: ProofMode,
    pub proof_provider: ProofProvider,
    pub call_guest_elf: GuestElf,
    pub chain_guest_ids: Box<[Digest]>,
    pub is_vlayer_test: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proof_mode: ProofMode::default(),
            proof_provider: ProofProvider::default(),
            call_guest_elf: GuestElf::default(),
            chain_guest_ids: vec![].into_boxed_slice(),
            is_vlayer_test: false,
        }
    }
}
