pub mod config;
pub mod error;

pub use config::HostConfig;
pub use error::HostError;
use host_utils::Prover;
use prove_chain_engine::Input;
use prove_chain_guest_wrapper::RISC0_CALL_GUEST_ELF;
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
    pub fn new(config: HostConfig) -> Result<Self, HostError> {
        let prover = Prover::new(config.proof_mode);

        Ok(Host { prover })
    }

    pub fn run(self) -> Result<HostOutput, HostError> {
        let input = Input {};

        let env = Self::build_executor_env(input)
            .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?;
        let ProveInfo { receipt, .. } = Self::prove(&self.prover, env, RISC0_CALL_GUEST_ELF)?;

        Ok(HostOutput { receipt })
    }

    fn prove(prover: &Prover, env: ExecutorEnv, guest_elf: &[u8]) -> Result<ProveInfo, HostError> {
        prover
            .prove(env, guest_elf)
            .map_err(|err| HostError::Prover(err.to_string()))
    }

    fn build_executor_env(input: impl Serialize) -> anyhow::Result<ExecutorEnv<'static>> {
        Ok(ExecutorEnv::builder().write(&input)?.build()?)
    }
}
