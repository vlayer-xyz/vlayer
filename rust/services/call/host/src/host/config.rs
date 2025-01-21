use common::GuestElf;
use host_utils::ProofMode;
use risc0_zkvm::sha::Digest;

pub struct Config {
    pub proof_mode: ProofMode,
    pub call_guest_elf: GuestElf,
    pub chain_guest_ids: Box<[Digest]>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proof_mode: ProofMode::default(),
            call_guest_elf: GuestElf::default(),
            chain_guest_ids: vec![].into_boxed_slice(),
        }
    }
}
