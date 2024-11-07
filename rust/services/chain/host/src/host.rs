pub mod config;
pub mod error;
mod prover;
mod strategy;

use alloy_primitives::ChainId;
use block_trie::BlockTrie;
use chain_db::{ChainDb, ChainTrie, ChainUpdate, Mode};
use chain_guest::Input;
use chain_guest_wrapper::RISC0_CHAIN_GUEST_ID;
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    middleware::Middleware,
    providers::{Http, JsonRpcClient, Provider},
    types::BlockNumber as BlockTag,
};
use futures::future::join_all;
use lazy_static::lazy_static;
use prover::Prover;
use provider::{to_eth_block_header, EvmBlockHeader};
use risc0_zkvm::sha::Digest;
use strategy::AppendPrependRanges;
pub use strategy::Strategy;
use tracing::{info, instrument};
use u64_range::{NonEmptyRange, Range};

lazy_static! {
    static ref GUEST_ID: Digest = RISC0_CHAIN_GUEST_ID.into();
}

pub struct Host<P>
where
    P: JsonRpcClient,
{
    db: ChainDb,
    prover: Prover,
    provider: Provider<P>,
    chain_id: ChainId,
    strategy: Strategy,
}

impl Host<Http> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider = Provider::<Http>::try_from(config.rpc_url.as_str())
            .expect("could not instantiate HTTP Provider");
        let prover = Prover::new(config.proof_mode);
        let db = ChainDb::mdbx(config.db_path, Mode::ReadWrite)?;

        Ok(Host::from_parts(prover, provider, db, config.chain_id, config.strategy))
    }
}

impl<P> Host<P>
where
    P: JsonRpcClient,
{
    pub const fn from_parts(
        prover: Prover,
        provider: Provider<P>,
        db: ChainDb,
        chain_id: ChainId,
        strategy: Strategy,
    ) -> Self {
        Host {
            prover,
            provider,
            db,
            chain_id,
            strategy,
        }
    }

    #[instrument(skip(self))]
    async fn poll(&self) -> Result<ChainUpdate, HostError> {
        match self.db.get_chain_info(self.chain_id)? {
            None => self.initialize().await,
            Some(_) => self.append_prepend().await,
        }
    }

    #[instrument(skip(self))]
    pub async fn poll_commit(&mut self) -> Result<(), HostError> {
        let chain_update = self.poll().await?;
        self.db.update_chain(self.chain_id, chain_update)?;
        Ok(())
    }

    #[instrument(skip(self))]
    async fn initialize(&self) -> Result<ChainUpdate, HostError> {
        info!("Initializing chain");
        let latest_block = self.get_block(BlockTag::Latest).await?;
        let latest_block_number = latest_block.number();
        let trie = BlockTrie::init(&latest_block)?;

        let input = Input::Initialize {
            elf_id: *GUEST_ID,
            block: latest_block,
        };
        let receipt = self.prover.prove(&input, None)?;

        let range = NonEmptyRange::from_single_value(latest_block_number);
        let chain_update = ChainUpdate::from_two_tries(range, vec![], &trie, &receipt)?;

        Ok(chain_update)
    }

    #[instrument(skip(self))]
    async fn append_prepend(&self) -> Result<ChainUpdate, HostError> {
        info!("Appending and prepending blocks");
        let ChainTrie {
            block_range: old_range,
            trie: old_trie,
            zk_proof: old_zk_proof,
        } = self
            .db
            .get_chain_trie(self.chain_id)?
            .expect("chain trie not found");
        let mut trie = old_trie.clone();

        let latest_block = self.get_block(BlockTag::Latest).await?;
        let latest_block_number = latest_block.number();
        let AppendPrependRanges {
            prepend,
            append,
            new_range,
            ..
        } = self
            .strategy
            .get_append_prepend_ranges(old_range, latest_block_number);
        let append_blocks = self.get_blocks_range(append).await?;
        let prepend_blocks = self.get_blocks_range(prepend).await?;
        let old_leftmost_block = self.get_block(old_range.start().into()).await?;

        trie.append(append_blocks.iter())?;
        trie.prepend(prepend_blocks.iter(), &old_leftmost_block)?;

        let input = Input::AppendPrepend {
            elf_id: *GUEST_ID,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            block_trie: old_trie.clone(),
        };
        let receipt = self.prover.prove(&input, Some(old_zk_proof))?;
        let chain_update = ChainUpdate::from_two_tries(new_range, &old_trie, &trie, &receipt)?;

        Ok(chain_update)
    }

    #[instrument(skip(self))]
    async fn get_blocks_range(
        &self,
        range: Range,
    ) -> Result<Vec<Box<dyn EvmBlockHeader>>, HostError> {
        let blocks = join_all(range.into_iter().map(|n| self.get_block(n.into()))).await;
        blocks.into_iter().collect()
    }

    #[instrument(skip(self))]
    async fn get_block(&self, number: BlockTag) -> Result<Box<dyn EvmBlockHeader>, HostError> {
        info!("Fetching block {}", number);
        let ethers_block = self
            .provider
            .get_block(number)
            .await
            .map_err(|err| HostError::Provider(err.to_string()))?
            .ok_or(HostError::BlockNotFound(number))?;
        let block = to_eth_block_header(ethers_block)
            .map_err(|e| HostError::BlockConversion(e.to_string()))?;
        info!("Fetched block {}", block.number());
        Ok(Box::new(block))
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;
    use serde_json::Value;

    use super::*;

    const MAX_HEAD_BLOCKS: u64 = 10;
    const MAX_BACK_PROPAGATION_BLOCKS: u64 = 10;
    const CONFIRMATIONS: u64 = 2;
    const LATEST: u64 = 500;

    lazy_static! {
        static ref STRATEGY: Strategy =
            Strategy::new(MAX_HEAD_BLOCKS, MAX_BACK_PROPAGATION_BLOCKS, CONFIRMATIONS);
    }

    fn test_db() -> ChainDb {
        ChainDb::in_memory()
    }

    mod poll {
        use chain_test_utils::mock_provider;
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
            let host = Host::from_parts(
                Prover::default(),
                mock_provider([LATEST]),
                db,
                1,
                STRATEGY.clone(),
            );

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
            const CONFIRMATIONS: u64 = 2;

            async fn test_db_after_initialize() -> Result<ChainDb, HostError> {
                let db = test_db();
                let host = Host::from_parts(
                    Prover::default(),
                    mock_provider([GENESIS]),
                    db,
                    1,
                    STRATEGY.clone(),
                );

                let init_chain_update = host.poll().await?;
                let Host { mut db, .. } = host;
                db.update_chain(1, init_chain_update).unwrap();

                Ok(db)
            }

            #[tokio::test]
            async fn no_new_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
                let db = test_db_after_initialize().await?;
                let host = Host::from_parts(
                    Prover::default(),
                    mock_provider([GENESIS, GENESIS]),
                    db,
                    1,
                    STRATEGY.clone(),
                );

                let chain_update = host.poll().await?;
                let Host { mut db, .. } = host;
                db.update_chain(1, chain_update)?;

                assert_fetched_latest_block(host.provider.as_ref());
                let chain_trie = db.get_chain_trie(1)?.unwrap();
                assert_eq!(chain_trie.block_range, GENESIS..=GENESIS);

                Ok(())
            }

            #[tokio::test]
            async fn new_confirmed_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
                let latest = GENESIS + CONFIRMATIONS;
                let new_confirmed_block = latest - CONFIRMATIONS + 1;
                let db = test_db_after_initialize().await?;
                let provider = mock_provider([latest, new_confirmed_block, GENESIS]);
                let host = Host::from_parts(Prover::default(), provider, db, 1, STRATEGY.clone());

                let chain_update = host.poll().await?;
                let Host { mut db, .. } = host;
                db.update_chain(1, chain_update)?;

                assert_fetched_latest_block(host.provider.as_ref());
                let chain_trie = db.get_chain_trie(1)?.unwrap();
                assert_eq!(chain_trie.block_range, GENESIS..=new_confirmed_block);

                Ok(())
            }

            // Reorg
        }
    }
}
