pub mod config;
pub mod error;

use alloy_primitives::{ChainId, B256};
use bytes::Bytes;
use chain_db::{ChainDb, ChainInfo, ChainUpdate};
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber as BlockTag,
};
use host_utils::Prover;
use lazy_static::lazy_static;
use provider::{to_eth_block_header, EvmBlockHeader};
use risc0_zkvm::{ExecutorEnv, ProveInfo};
use serde::Serialize;

lazy_static! {
    static ref EMPTY_PROOF: Bytes = Bytes::new();
}

pub struct Host<P>
where
    P: JsonRpcClient,
{
    _prover: Prover,
    provider: Provider<P>,
    db: ChainDb,
    chain_id: ChainId,
}

impl Host<Http> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider = Provider::<Http>::try_from(config.rpc_url.as_str())
            .expect("could not instantiate HTTP Provider");
        let prover = Prover::new(config.proof_mode);
        let db = ChainDb::new(config.db_path)?;

        Ok(Host::from_parts(prover, provider, db, config.chain_id))
    }
}

impl<P> Host<P>
where
    P: JsonRpcClient,
{
    pub fn from_parts(
        prover: Prover,
        provider: Provider<P>,
        db: ChainDb,
        chain_id: ChainId,
    ) -> Self {
        Host {
            _prover: prover,
            provider,
            db,
            chain_id,
        }
    }

    pub async fn poll(&self) -> Result<ChainUpdate, HostError> {
        match self.db.get_chain_info(self.chain_id)? {
            None => self.initialize().await,
            Some(chain_info) => self.append_prepend(chain_info).await,
        }
    }

    async fn initialize(&self) -> Result<ChainUpdate, HostError> {
        let block = self.get_block(BlockTag::Latest).await?;
        let range = block.number()..=block.number();
        let chain_info = ChainInfo::new(range, B256::ZERO, EMPTY_PROOF.clone());
        let chain_update = ChainUpdate::new(chain_info, [], []);
        Ok(chain_update)
    }

    async fn append_prepend(
        &self,
        current_chain_info: ChainInfo,
    ) -> Result<ChainUpdate, HostError> {
        let chain_update = ChainUpdate::new(current_chain_info, [], []);
        Ok(chain_update)
    }

    async fn get_block(&self, number: BlockTag) -> Result<Box<dyn EvmBlockHeader>, HostError> {
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
mod test_utils;

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
        use chain_db::InMemoryDatabase;
        use serde_json::Value;
        use test_utils::mock_provider;

        use super::*;

        fn test_db() -> ChainDb {
            ChainDb::from_db(InMemoryDatabase::new())
        }

        mod poll {
            use super::*;

            #[tokio::test]
            async fn initialize() -> anyhow::Result<()> {
                let host =
                    Host::from_parts(Prover::default(), mock_provider([20_000_000]), test_db(), 1);

                let chain_update = host.poll().await?;

                host.provider.as_ref().assert_request(
                    "eth_getBlockByNumber",
                    Value::Array(vec!["latest".into(), false.into()]),
                )?;
                assert_eq!(
                    chain_update,
                    ChainUpdate::new(
                        ChainInfo::new(20_000_000..=20_000_000, B256::ZERO, EMPTY_PROOF.clone()),
                        [],
                        []
                    )
                );

                Ok(())
            }

            mod append_prepend {
                use alloy_primitives::BlockNumber;

                use super::*;
                const GENERIS_BLOCK_NUMBER: BlockNumber = 0;

                #[tokio::test]
                async fn no_new_work_back_propagation_finished() -> anyhow::Result<()> {
                    let mut db = test_db();
                    let chain_info = ChainInfo::new(
                        GENERIS_BLOCK_NUMBER..=20_000_000,
                        B256::ZERO,
                        EMPTY_PROOF.clone(),
                    );
                    let chain_update = ChainUpdate::new(chain_info.clone(), [], []);
                    db.update_chain(1, chain_update)?;
                    let host =
                        Host::from_parts(Prover::default(), mock_provider([20_000_000]), db, 1);

                    let chain_update = host.poll().await?;

                    assert_eq!(chain_update, ChainUpdate::new(chain_info, [], []));

                    Ok(())
                }
                // No new work
                // New head blocks, back propagation finished
                // New head blocks, back propagation in progress
                // Too many new blocks
                // Reorg
            }
        }
    }
}
