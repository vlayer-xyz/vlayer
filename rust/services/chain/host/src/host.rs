pub mod config;
pub mod error;

use alloy_primitives::{BlockNumber, ChainId};
use chain_engine::Input;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ELF;
pub use config::HostConfig;
pub use error::HostError;
use host_utils::Prover;
use mpt::MerkleTrie;
use risc0_zkvm::{ExecutorEnv, ProveInfo, Receipt};
use serde::Serialize;

pub struct Host {
    prover: Prover,
}

pub struct HostOutput {
    pub receipt: Receipt,
}

impl Host {
    pub fn new(config: &HostConfig) -> Self {
        let prover = Prover::new(config.proof_mode);

        Host { prover }
    }

    pub fn run(
        self,
        _chain_id: ChainId,
        _block_numbers: &[BlockNumber],
        merkle_trie: &MerkleTrie,
    ) -> Result<HostOutput, HostError> {
        let root_hash = merkle_trie.hash_slow();
        let input = Input { root_hash };

        let env = build_executor_env(input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        let ProveInfo { receipt, .. } = provably_execute(&self.prover, env, RISC0_CHAIN_GUEST_ELF)?;

        Ok(HostOutput { receipt })
    }
}

fn provably_execute(
    prover: &Prover,
    env: ExecutorEnv,
    guest_elf: &[u8],
) -> Result<ProveInfo, HostError> {
    prover
        .prove(env, guest_elf)
        .map_err(|err| HostError::Prover(err.to_string()))
}

fn build_executor_env(input: impl Serialize) -> anyhow::Result<ExecutorEnv<'static>> {
    ExecutorEnv::builder().write(&input)?.build()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn host_prove_invalid_guest_elf() {
        let prover = Prover::default();
        let env = ExecutorEnv::default();
        let invalid_guest_elf = &[];
        let res = provably_execute(&prover, env, invalid_guest_elf);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }
}
