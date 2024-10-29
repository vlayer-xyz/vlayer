pub mod config;
pub mod error;
mod prover;

use std::ops::RangeInclusive;

use alloy_primitives::ChainId;
use block_trie::{BlockTrie, EMPTY_TRIE};
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
use tracing::{info, instrument};

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
}

impl Host<Http> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let provider = Provider::<Http>::try_from(config.rpc_url.as_str())
            .expect("could not instantiate HTTP Provider");
        let prover = Prover::new(config.proof_mode);
        let db = ChainDb::mdbx(config.db_path, Mode::ReadWrite)?;

        Ok(Host::from_parts(prover, provider, db, config.chain_id))
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
    ) -> Self {
        Host {
            prover,
            provider,
            db,
            chain_id,
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
        let trie = BlockTrie::init(&*latest_block)?;

        let input = Input::Initialize {
            elf_id: *GUEST_ID,
            block: latest_block,
        };
        let receipt = self.prover.prove(input, None)?;

        let range = latest_block_number..=latest_block_number;
        let chain_update = ChainUpdate::from_two_tries(range, &EMPTY_TRIE, &trie, receipt);

        Ok(chain_update)
    }

    #[instrument(skip(self))]
    async fn append_prepend(&self) -> Result<ChainUpdate, HostError> {
        info!("Appending and prepending blocks");
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

        for block in &append_blocks {
            trie.append(block.as_ref())?;
        }

        let input = Input::AppendPrepend {
            elf_id: *GUEST_ID,
            prepend_blocks: vec![],
            append_blocks,
            old_leftmost_block: latest_block,
            block_trie: old_trie.clone(),
        };
        let receipt = self.prover.prove(input, Some(old_zk_proof))?;

        let range = *block_range.start()..=latest_block_number;
        let chain_update = ChainUpdate::from_two_tries(range, &old_trie, &trie, receipt);

        Ok(chain_update)
    }

    #[instrument(skip(self))]
    async fn get_blocks_range(
        &self,
        range: RangeInclusive<u64>,
    ) -> Result<Vec<Box<dyn EvmBlockHeader>>, HostError> {
        let blocks = join_all(range.map(|n| self.get_block(n.into()))).await;
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
mod test_utils;

#[cfg(test)]
mod test {

    use serde_json::Value;
    use test_utils::mock_provider;

    use super::*;

    const LATEST: u64 = 20_000_000;

    fn test_db() -> ChainDb {
        ChainDb::in_memory()
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
