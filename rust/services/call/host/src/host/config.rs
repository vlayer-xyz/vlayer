use common::GuestElf;
use host_utils::ProofMode;

pub const DEFAULT_MAX_CALLDATA_SIZE: usize = 5 * 1024 * 1024; // 5 MB

pub struct Config {
    pub proof_mode: ProofMode,
    pub max_calldata_size: usize,
    pub call_guest_elf: GuestElf,
    pub chain_guest_elf: GuestElf,
    pub verify_chain_proofs: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proof_mode: ProofMode::default(),
            max_calldata_size: DEFAULT_MAX_CALLDATA_SIZE,
            call_guest_elf: GuestElf::default(),
            chain_guest_elf: GuestElf::default(),
            verify_chain_proofs: false,
        }
    }
}
