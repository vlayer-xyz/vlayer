pub mod config;
pub mod error;

use alloy_primitives::ChainId;
use chain_db::Database;
use chain_engine::Input;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ELF;
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    providers::{Http, Middleware, Provider},
    types::BlockNumber,
};
use host_utils::Prover;
use provider::to_eth_block_header;
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

    pub async fn initialize<'db>(
        self,
        _chain_id: ChainId,
        provider: &Provider<Http>,
        _db: &mut impl Database<'db>,
    ) -> Result<HostOutput, HostError> {
        let ethers_block = provider
            .get_block(BlockNumber::Latest)
            .await
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::NoLatestBlock)?;

        let block = to_eth_block_header(ethers_block).map_err(HostError::BlockConversion)?;

        let input = Input::Initialize {
            block: Box::new(block),
        };

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
