pub mod config;
pub mod error;

use std::ops::RangeInclusive;

use alloy_primitives::ChainId;
use block_trie::BlockTrie;
use bytes::Bytes;
use chain_db::{difference, ChainDb, ChainInfo, ChainTrie, ChainUpdate};
use chain_guest::Input;
use chain_guest_wrapper::{RISC0_CHAIN_GUEST_ELF, RISC0_CHAIN_GUEST_ID};
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber as BlockTag,
};
use futures::future::join_all;
use host_utils::Prover;
use lazy_static::lazy_static;
use provider::{to_eth_block_header, EvmBlockHeader};
use risc0_zkvm::{sha::Digest, ExecutorEnv, ProveInfo, Receipt};
use serde::Serialize;

lazy_static! {
    static ref GUEST_ID: Digest = RISC0_CHAIN_GUEST_ID.into();
}

pub struct Host<P>
where
    P: JsonRpcClient,
{
    prover: Prover,
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
            prover,
            provider,
            db,
            chain_id,
        }
    }

    pub async fn poll(&self) -> Result<ChainUpdate, HostError> {
        match self.db.get_chain_info(self.chain_id)? {
            None => self.initialize().await,
            Some(_) => self.append_prepend().await,
        }
    }

    async fn initialize(&self) -> Result<ChainUpdate, HostError> {
        let latest_block = self.get_block(BlockTag::Latest).await?;
        let latest_block_number = latest_block.number();
        let block_trie = BlockTrie::init(&*latest_block);

        let input = Input::Initialize {
            elf_id: *GUEST_ID,
            block: latest_block,
        };
        let receipt = self.prove(input)?;
        let zk_proof = encode_proof(&receipt);

        let range = latest_block_number..=latest_block_number;
        let chain_info = ChainInfo::new(range, block_trie.hash_slow(), zk_proof);
        let chain_update = ChainUpdate::new(chain_info, &block_trie, []);

        Ok(chain_update)
    }

    async fn append_prepend(&self) -> Result<ChainUpdate, HostError> {
        let ChainTrie {
            block_range,
            trie: old_trie,
            zk_proof: old_zk_proof,
        } = self
            .db
            .get_chain_trie(self.chain_id)?
            .expect("chain trie not found");
        let mut trie = old_trie.clone();

        let latest_block = self.get_block(BlockTag::Latest).await?;
        let latest_block_number = latest_block.number();
        let append_range = block_range.end() + 1..=latest_block_number;
        let append_blocks = self.get_blocks_range(append_range).await?;

        for block in append_blocks {
            // SAFETY: UNSAFE - It's a stub to makes the partial test pass
            trie.insert_unchecked(block.number(), &block.hash_slow());
        }

        let block_range = *block_range.start()..=latest_block_number;
        let chain_info = ChainInfo::new(block_range, trie.hash_slow(), old_zk_proof);
        let (added, removed) = difference(&old_trie, &trie);
        let chain_update = ChainUpdate::new(chain_info, added, removed);

        Ok(chain_update)
    }

    fn prove(&self, input: Input) -> Result<Receipt, HostError> {
        let executor_env = build_executor_env(input)?;
        let ProveInfo { receipt, .. } =
            provably_execute(&self.prover, executor_env, RISC0_CHAIN_GUEST_ELF)?;
        Ok(receipt)
    }

    async fn get_blocks_range(
        &self,
        range: RangeInclusive<u64>,
    ) -> Result<Vec<Box<dyn EvmBlockHeader>>, HostError> {
        let blocks = join_all(range.map(|n| self.get_block(n.into()))).await;
        blocks.into_iter().collect()
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

fn encode_proof(receipt: &Receipt) -> Bytes {
    bincode::serialize(receipt)
        .expect("failed to serialize receipt")
        .into()
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

fn build_executor_env(input: impl Serialize) -> Result<ExecutorEnv<'static>, HostError> {
    ExecutorEnv::builder()
        .write(&input)
        .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))?
        .build()
        .map_err(|err| HostError::ExecutorEnvBuilder(err.to_string()))
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
        use key_value::InMemoryDatabase;
        use serde_json::Value;
        use test_utils::mock_provider;

        use super::*;

        const LATEST: u64 = 20_000_000;

        fn test_db() -> ChainDb {
            ChainDb::from_db(InMemoryDatabase::new())
        }

        mod poll {
            use ethers::providers::MockProvider;

            use super::*;

            fn assert_fetched_latest_block(provider: &MockProvider) {
                provider
                    .assert_request(
                        "eth_getBlockByNumber",
                        Value::Array(vec!["latest".into(), false.into()]),
                    )
                    .expect("expected request to fetch latest block");
            }

            #[tokio::test]
            async fn initialize() -> anyhow::Result<()> {
                let db = test_db();
                let host = Host::from_parts(Prover::default(), mock_provider([LATEST]), db, 1);

                let chain_update = host.poll().await?;
                let Host { mut db, .. } = host;
                db.update_chain(1, chain_update)?;

                assert_fetched_latest_block(host.provider.as_ref());
                let chain_trie = db.get_chain_trie(1)?.unwrap();
                assert_eq!(chain_trie.block_range, LATEST..=LATEST);

                Ok(())
            }

            mod append_prepend {

                use alloy_primitives::BlockNumber;

                use super::*;
                const GENESIS: BlockNumber = 0;

                async fn test_db_after_initialize() -> Result<ChainDb, HostError> {
                    let db = test_db();
                    let host = Host::from_parts(Prover::default(), mock_provider([GENESIS]), db, 1);

                    let init_chain_update = host.poll().await?;
                    let Host { mut db, .. } = host;
                    db.update_chain(1, init_chain_update).unwrap();

                    Ok(db)
                }

                #[tokio::test]
                async fn no_new_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
                    let db = test_db_after_initialize().await?;
                    let host = Host::from_parts(Prover::default(), mock_provider([GENESIS]), db, 1);

                    let chain_update = host.poll().await?;
                    let Host { mut db, .. } = host;
                    db.update_chain(1, chain_update)?;

                    assert_fetched_latest_block(host.provider.as_ref());
                    let chain_trie = db.get_chain_trie(1)?.unwrap();
                    assert_eq!(chain_trie.block_range, GENESIS..=GENESIS);

                    Ok(())
                }

                #[tokio::test]
                async fn new_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
                    let new_block = GENESIS + 1;
                    let db = test_db_after_initialize().await?;
                    let provider = mock_provider([new_block, new_block]);
                    let host = Host::from_parts(Prover::default(), provider, db, 1);

                    let chain_update = host.poll().await?;
                    let Host { mut db, .. } = host;
                    db.update_chain(1, chain_update)?;

                    assert_fetched_latest_block(host.provider.as_ref());
                    let chain_trie = db.get_chain_trie(1)?.unwrap();
                    assert_eq!(chain_trie.block_range, GENESIS..=GENESIS + 1);

                    Ok(())
                }

                // No new head blocks, back propagation in progress
                // New head blocks, back propagation in progress
                // Too many new head blocks
                // Reorg
            }
        }
    }
}
