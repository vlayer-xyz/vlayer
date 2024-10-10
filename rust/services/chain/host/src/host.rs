pub mod config;
pub mod error;

use alloy_primitives::ChainId;
use chain_guest::Input;
use chain_guest_wrapper::{RISC0_CHAIN_GUEST_ELF, RISC0_CHAIN_GUEST_ID};
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
            elf_id: RISC0_CHAIN_GUEST_ID.into(),
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
    mod provably_execute {
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
    mod host {
        use std::sync::Arc;

        use alloy_primitives::B256;
        use alloy_rlp::encode_fixed_size;
        use ethers::{providers::Provider, types::Block};
        use lazy_static::lazy_static;
        use mpt::MerkleTrie;
        use provider::EvmBlockHeader;
        use risc0_zkp::core::digest::Digest;
        use serde_json::{from_value, json, Value};

        use super::*;

        lazy_static! {
            // All fields are zeroed out except for the block number
            static ref rpc_block: Block<()> = from_value(json!(
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
            static ref block: Arc<dyn EvmBlockHeader> = Arc::new(to_eth_block_header(rpc_block.clone()).unwrap());
            static ref block_hash: B256 = block.hash_slow();

            static ref config: HostConfig = HostConfig::default();
        }

        #[tokio::test]
        async fn initialize() -> anyhow::Result<()> {
            let (provider, mock) = Provider::mocked();
            mock.push(rpc_block.clone())?;

            let encoded_block_num = encode_fixed_size(&block.number());
            let expected_root_hash =
                MerkleTrie::from_iter([(encoded_block_num, *block_hash)]).hash_slow();

            let host = Host::new(&config);
            let HostOutput { receipt } = host.initialize(ChainId::default(), &provider).await?;

            mock.assert_request(
                "eth_getBlockByNumber",
                Value::Array(vec!["latest".into(), false.into()]),
            )?;

            let (root_hash, elf_id): (B256, Digest) = receipt.journal.decode()?;

            assert_eq!(root_hash, expected_root_hash);
            assert_eq!(elf_id, RISC0_CHAIN_GUEST_ID.into());

            Ok(())
        }
    }
}
