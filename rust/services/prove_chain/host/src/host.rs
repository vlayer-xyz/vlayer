pub mod config;
pub mod error;

pub use config::HostConfig;
pub use error::HostError;
use host_utils::Prover;
use prove_chain_engine::Input;
use prove_chain_guest_wrapper::RISC0_PROVE_CHAIN_GUEST_ELF;
use risc0_zkvm::{ExecutorEnv, ProveInfo, Receipt};
use serde::Serialize;

pub struct Host {
    prover: Prover,
}

pub struct HostOutput {
    #[allow(unused)]
    receipt: Receipt,
}

impl Host {
    pub fn new(config: HostConfig) -> Self {
        let prover = Prover::new(config.proof_mode);

        Host { prover }
    }

    pub fn run(self) -> Result<HostOutput, HostError> {
        let input = Input {};

        let env = build_executor_env(input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        let ProveInfo { receipt, .. } = prove(&self.prover, env, RISC0_PROVE_CHAIN_GUEST_ELF)?;

        Ok(HostOutput { receipt })
    }
}

fn prove(prover: &Prover, env: ExecutorEnv, guest_elf: &[u8]) -> Result<ProveInfo, HostError> {
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
        let res = prove(&prover, env, invalid_guest_elf);

        assert!(matches!(
            res.map(|_| ()).unwrap_err(),
            HostError::Prover(ref msg) if msg == "Elf parse error: Could not read bytes in range [0x0, 0x10)"
        ));
    }
}
