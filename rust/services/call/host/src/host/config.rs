use common::GuestElf;
use host_utils::ProofMode;

pub struct Config {
    pub proof_mode: ProofMode,
    pub call_guest_elf: GuestElf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            proof_mode: ProofMode::default(),
            call_guest_elf: GuestElf::default(),
        }
    }
}
