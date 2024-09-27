pub mod config;
pub mod error;

use alloy_primitives::ChainId;
use chain_engine::Input;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ELF;
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    providers::{JsonRpcClient, Middleware, Provider},
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

    pub async fn initialize<P: JsonRpcClient>(
        self,
        _chain_id: ChainId,
        provider: &Provider<P>,
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
    use ethers::{providers::Provider, types::Block};
    use lazy_static::lazy_static;
    use serde_json::{from_value, json};

    lazy_static! {
        static ref block: Block<()> = from_value(json!(
        {
            "number": "0x42",

            "baseFeePerGas": "0x0",
            "miner": "0x0000000000000000000000000000000000000000",
            "hash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "nonce": "0x0000000000000000",
            "sealFields": [],
            "sha3Uncles": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            "transactionsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "receiptsRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "stateRoot": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "difficulty": "0x0",
            "totalDifficulty": "0x0",
            "extraData": "0x0000000000000000000000000000000000000000000000000000000000000000",
            "size": "0x0",
            "gasLimit": "0x0",
            "minGasPrice": "0x0",
            "gasUsed": "0x0",
            "timestamp": "0x0",
            "transactions": [],
            "uncles": []
          }
        )).unwrap();
    }

    #[tokio::test]
    async fn initialize() -> anyhow::Result<()> {
        let config = HostConfig::default();
        let host = Host::new(&config);

        let (provider, mock) = Provider::mocked();
        mock.push(block.clone())?;

        let HostOutput { receipt } = host.initialize(ChainId::default(), &provider).await?;

        let actual_block_number: u64 = receipt.journal.decode()?;
        assert_eq!(actual_block_number, 0x42);

        Ok(())
    }

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
