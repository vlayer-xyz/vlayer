mod block_fetcher;
pub mod config;
pub mod error;
mod prover;
mod strategy;

use alloy_primitives::ChainId;
use block_fetcher::BlockFetcher;
use block_trie::BlockTrie;
use chain_db::{ChainDb, ChainTrie, ChainUpdate, Mode};
use chain_guest::Input;
use chain_guest_wrapper::GUEST_ELF;
pub use config::HostConfig;
pub use error::HostError;
use ethers::{
    providers::{Http, JsonRpcClient},
    types::BlockNumber as BlockTag,
};
use prover::Prover;
pub use strategy::{AppendStrategy, PrependStrategy};
use tracing::{info, instrument};
use u64_range::NonEmptyRange;

pub struct Host<P>
where
    P: JsonRpcClient,
{
    db: ChainDb,
    prover: Prover,
    fetcher: BlockFetcher<P>,
    chain_id: ChainId,
    prepend_strategy: PrependStrategy,
    append_strategy: AppendStrategy,
}

impl Host<Http> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let block_fetcher = BlockFetcher::<Http>::new(config.rpc_url)?;
        let prover = Prover::new(config.proof_mode);
        let db = ChainDb::mdbx(config.db_path, Mode::ReadWrite)?;

        Ok(Host::from_parts(
            prover,
            block_fetcher,
            db,
            config.chain_id,
            config.prepend_strategy,
            config.append_strategy,
        ))
    }
}

impl<P> Host<P>
where
    P: JsonRpcClient,
{
    pub const fn from_parts(
        prover: Prover,
        block_fetcher: BlockFetcher<P>,
        db: ChainDb,
        chain_id: ChainId,
        prepend_strategy: PrependStrategy,
        append_strategy: AppendStrategy,
    ) -> Self {
        Host {
            prover,
            fetcher: block_fetcher,
            db,
            chain_id,
            prepend_strategy,
            append_strategy,
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
        let latest_block = self.fetcher.get_block(BlockTag::Latest).await?;
        let latest_block_number = latest_block.number();
        let trie = BlockTrie::init(&latest_block)?;

        let input = Input::Initialize {
            elf_id: GUEST_ELF.id,
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

        let latest_block = self.fetcher.get_block(BlockTag::Latest).await?;

        let (new_range, prepend) = self.prepend_strategy.compute_prepend_range(old_range);
        let (new_range, append) = self
            .append_strategy
            .compute_append_range(new_range, latest_block.number());
        let prepend_blocks = self.fetcher.get_blocks_range(prepend).await?;
        let append_blocks = self.fetcher.get_blocks_range(append).await?;
        let old_leftmost_block = self.fetcher.get_block(old_range.start().into()).await?;

        trie.prepend(prepend_blocks.iter(), &old_leftmost_block)?;
        trie.append(append_blocks.iter())?;

        let input = Input::AppendPrepend {
            elf_id: GUEST_ELF.id,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            block_trie: old_trie.clone(),
        };
        let receipt = self.prover.prove(&input, Some(old_zk_proof))?;
        let chain_update = ChainUpdate::from_two_tries(new_range, &old_trie, &trie, &receipt)?;

        Ok(chain_update)
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::BlockNumber;
    use chain_test_utils::mock_provider;
    use ethers::providers::{MockProvider, Provider};
    use lazy_static::lazy_static;

    use super::*;

    const MAX_HEAD_BLOCKS: u64 = 10;
    const MAX_BACK_PROPAGATION_BLOCKS: u64 = 10;
    const CONFIRMATIONS: u64 = 2;
    const LATEST: u64 = 500;
    const GENESIS: BlockNumber = 0;

    lazy_static! {
        static ref PREPEND_STRATEGY: PrependStrategy =
            PrependStrategy::new(MAX_BACK_PROPAGATION_BLOCKS);
    }

    lazy_static! {
        static ref APPEND_STRATEGY: AppendStrategy =
            AppendStrategy::new(MAX_HEAD_BLOCKS, CONFIRMATIONS);
    }

    fn test_db() -> ChainDb {
        ChainDb::in_memory()
    }

    fn create_host(db: ChainDb, provider: Provider<MockProvider>) -> Host<MockProvider> {
        Host::from_parts(
            Prover::default(),
            BlockFetcher::from_provider(provider),
            db,
            1,
            PREPEND_STRATEGY.clone(),
            APPEND_STRATEGY.clone(),
        )
    }

    #[tokio::test]
    async fn initialize() -> anyhow::Result<()> {
        let host = create_host(test_db(), mock_provider([LATEST]));

        let chain_update = host.poll().await?;
        let Host { mut db, .. } = host;
        db.update_chain(1, chain_update)?;

        let chain_trie = db.get_chain_trie(1)?.unwrap();
        assert_eq!(chain_trie.block_range, LATEST..=LATEST);

        Ok(())
    }

    mod append_prepend {
        use super::*;

        async fn db_after_initialize() -> Result<ChainDb, HostError> {
            let host = create_host(test_db(), mock_provider([GENESIS]));

            let init_chain_update = host.poll().await?;
            let Host { mut db, .. } = host;
            db.update_chain(1, init_chain_update).unwrap();

            Ok(db)
        }

        #[tokio::test]
        async fn no_new_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
            let host = create_host(db_after_initialize().await?, mock_provider([GENESIS, GENESIS]));

            let chain_update = host.poll().await?;
            let Host { mut db, .. } = host;
            db.update_chain(1, chain_update)?;

            let chain_trie = db.get_chain_trie(1)?.unwrap();
            assert_eq!(chain_trie.block_range, GENESIS..=GENESIS);

            Ok(())
        }

        #[tokio::test]
        async fn new_confirmed_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
            let latest = GENESIS + CONFIRMATIONS;
            let new_confirmed_block = latest - CONFIRMATIONS + 1;
            let host = create_host(
                db_after_initialize().await?,
                mock_provider([latest, new_confirmed_block, GENESIS]),
            );

            let chain_update = host.poll().await?;
            let Host { mut db, .. } = host;
            db.update_chain(1, chain_update)?;

            let chain_trie = db.get_chain_trie(1)?.unwrap();
            assert_eq!(chain_trie.block_range, GENESIS..=new_confirmed_block);

            Ok(())
        }

        // Reorg
    }
}
