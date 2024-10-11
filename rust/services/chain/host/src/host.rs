pub mod config;
pub mod error;

use alloy_primitives::B256;
use chain_db::{ChainDb, ChainInfo, ChainUpdate, Database, Mdbx};
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber,
};
use host_utils::Prover;
use lazy_static::lazy_static;
use provider::{to_eth_block_header, EvmBlockHeader};
use risc0_zkvm::{ExecutorEnv, ProveInfo};
use serde::Serialize;

lazy_static! {
    static ref EMPTY_PROOF: Vec<u8> = vec![];
}

pub struct Host<P, DB>
where
    P: JsonRpcClient,
    DB: for<'a> Database<'a>,
{
    _prover: Prover,
    provider: Provider<P>,
    _db: ChainDb<DB>,
}

impl Host<Http, Mdbx> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider = Provider::<Http>::try_from(config.rpc_url.as_str())
            .expect("could not instantiate HTTP Provider");
        let prover = Prover::new(config.proof_mode);
        let db = ChainDb::new(config.db_path)?;

        Ok(Host::from_parts(prover, provider, db))
    }
}

impl<P, DB> Host<P, DB>
where
    P: JsonRpcClient,
    DB: for<'a> Database<'a>,
{
    pub fn from_parts(prover: Prover, provider: Provider<P>, db: ChainDb<DB>) -> Self {
        Host {
            _prover: prover,
            provider,
            _db: db,
        }
    }

    pub async fn poll(&self) -> Result<ChainUpdate, HostError> {
        self.initialize().await
    }

    async fn initialize(&self) -> Result<ChainUpdate, HostError> {
        let block = self.get_block(BlockNumber::Latest).await?;
        let chain_info =
            ChainInfo::new(block.number()..=block.number(), B256::ZERO, EMPTY_PROOF.as_slice());
        let chain_update = ChainUpdate::new(chain_info, [], []);
        Ok(chain_update)
    }

    async fn get_block(&self, number: BlockNumber) -> Result<Box<dyn EvmBlockHeader>, HostError> {
        let ethers_block = self
            .provider
            .get_block(number)
            .await
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(number))?;
        let block = to_eth_block_header(ethers_block).map_err(HostError::BlockConversion)?;
        Ok(Box::new(block))
    }
}

#[allow(unused)]
fn provably_execute(
    prover: &Prover,
    env: ExecutorEnv,
    guest_elf: &[u8],
) -> Result<ProveInfo, HostError> {
    prover
        .prove(env, guest_elf)
        .map_err(|err| HostError::Prover(err.to_string()))
}

fn _build_executor_env(input: impl Serialize) -> anyhow::Result<ExecutorEnv<'static>> {
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
    mod host_poll {

        use alloy_primitives::BlockNumber;
        use chain_db::InMemoryDatabase;
        use ethers::{
            providers::{MockProvider, Provider},
            types::Block,
        };
        use serde_json::{from_value, json, Value};

        use super::*;

        fn fake_rpc_block(number: BlockNumber) -> Block<()> {
            // All fields are zeroed out except for the block number
            from_value(json!({
              "number": format!("{:x}", number),
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
            })).unwrap()
        }

        fn mock_provider(
            block_numbers: impl IntoIterator<Item = BlockNumber>,
        ) -> Provider<MockProvider> {
            let (provider, mock) = Provider::mocked();
            for block_number in block_numbers {
                mock.push(fake_rpc_block(block_number))
                    .expect("could not push block");
            }
            provider
        }

        #[tokio::test]
        async fn initialize() -> anyhow::Result<()> {
            let latest_block = 20_000_000;
            let host = Host::from_parts(
                Prover::default(),
                mock_provider([latest_block]),
                ChainDb::from_db(InMemoryDatabase::new()),
            );

            let chain_update = host.poll().await?;

            let mock = host.provider.as_ref();
            mock.assert_request(
                "eth_getBlockByNumber",
                Value::Array(vec!["latest".into(), false.into()]),
            )?;
            assert_eq!(
                chain_update,
                ChainUpdate::new(
                    ChainInfo::new(latest_block..=latest_block, B256::ZERO, EMPTY_PROOF.as_slice()),
                    [],
                    []
                )
            );

            Ok(())
        }

        mod append_prepend {
            // No new work
            // New head blocks, back propagation finished
            // New head blocks, back propagation in progress
            // Too many new blocks
            // Reorg
        }
    }
}
