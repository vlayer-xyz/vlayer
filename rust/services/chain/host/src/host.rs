mod block_fetcher;
pub mod config;
pub mod error;
mod prover;
mod strategy;

use std::time::Duration;

use alloy_primitives::ChainId;
use block_fetcher::BlockFetcher;
use block_trie::BlockTrie;
use chain_db::{ChainDb, ChainTrie, ChainUpdate, Mode};
use chain_guest::Input;
use common::GuestElf;
pub use config::HostConfig;
use error::HostError;
use ethers::{
    providers::{Http, JsonRpcClient},
    types::BlockNumber as BlockTag,
};
use prover::Prover;
pub use strategy::{AppendStrategy, PrependStrategy};
use tokio::time::sleep;
use tracing::{info, instrument};
use u64_range::NonEmptyRange;

const SLEEP_IF_FULLY_SYNCED: Duration = Duration::from_secs(1);

pub struct Host<P>
where
    P: JsonRpcClient,
{
    db: ChainDb,
    prover: Prover,
    fetcher: BlockFetcher<P>,
    chain_id: ChainId,
    elf: GuestElf,
    start_block: BlockTag,
    prepend_strategy: PrependStrategy,
    append_strategy: AppendStrategy,
}

impl Host<Http> {
    pub fn try_new(config: HostConfig) -> Result<Self, HostError> {
        let block_fetcher = BlockFetcher::<Http>::new(config.rpc_url)?;
        let prover = Prover::try_new(config.proof_mode, config.elf.clone())?;
        let db = ChainDb::mdbx(config.db_path, Mode::ReadWrite, config.chain_guest_ids.clone())?;

        Ok(Host::from_parts(
            prover,
            block_fetcher,
            db,
            config.chain_id,
            config.elf,
            config.start_block,
            config.prepend_strategy,
            config.append_strategy,
        ))
    }
}

impl<P> Host<P>
where
    P: JsonRpcClient,
{
    #[allow(clippy::too_many_arguments)]
    pub const fn from_parts(
        prover: Prover,
        block_fetcher: BlockFetcher<P>,
        db: ChainDb,
        chain_id: ChainId,
        elf: GuestElf,
        start_block: BlockTag,
        prepend_strategy: PrependStrategy,
        append_strategy: AppendStrategy,
    ) -> Self {
        Host {
            prover,
            fetcher: block_fetcher,
            db,
            chain_id,
            elf,
            start_block,
            prepend_strategy,
            append_strategy,
        }
    }

    #[instrument(skip(self))]
    async fn poll(&self) -> Result<Option<ChainUpdate>, HostError> {
        match self.db.get_chain_info(self.chain_id)? {
            None => Ok(Some(self.initialize().await?)),
            Some(_) => Ok(self.append_prepend().await?),
        }
    }

    #[instrument(skip(self))]
    pub async fn poll_commit(&mut self) -> Result<(), HostError> {
        if let Some(chain_update) = self.poll().await? {
            self.commit(chain_update)?;
        } else {
            sleep(SLEEP_IF_FULLY_SYNCED).await;
        };
        Ok(())
    }

    #[instrument(skip_all)]
    pub fn commit(&mut self, chain_update: ChainUpdate) -> Result<(), HostError> {
        let chain_info = &chain_update.chain_info;
        info!(
            first_block = chain_info.first_block,
            last_block = chain_info.last_block,
            root_hash = %chain_info.root_hash,
            chain_id = self.chain_id,
            guest_id = %self.elf.id,
            added_nodes = chain_update.added_nodes.len(),
            removed_nodes = chain_update.removed_nodes.len(),
            "Committing chain update to the database"
        );
        self.db.update_chain(self.chain_id, chain_update)?;
        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn initialize(&self) -> Result<ChainUpdate, HostError> {
        info!("Initializing chain");
        let start_block = self.fetcher.get_block(self.start_block).await?;
        let start_block_number = start_block.number();
        let trie = BlockTrie::init(&start_block)?;

        let input = Input::Initialize {
            elf_id: self.elf.id,
            block: start_block,
        };
        let receipt = self.prover.prove(&input, None)?;

        let range = NonEmptyRange::from_single_value(start_block_number);
        let chain_update = ChainUpdate::from_two_tries(range, vec![], &trie, &receipt)?;

        Ok(chain_update)
    }

    #[instrument(skip(self))]
    pub async fn append_prepend(&self) -> Result<Option<ChainUpdate>, HostError> {
        let ChainTrie {
            block_range: old_range,
            trie: old_trie,
            zk_proof: old_zk_proof,
        } = self.db.get_chain_trie(self.chain_id)?;

        let mut trie = old_trie.clone();

        let latest_block_number = self.fetcher.get_latest_block_number().await?;

        let (new_range, prepend) = self.prepend_strategy.compute_prepend_range(old_range);
        let (new_range, append) = self
            .append_strategy
            .compute_append_range(new_range, latest_block_number);

        if new_range == old_range {
            info!("No new blocks to append or prepend");
            return Ok(None);
        }
        info!(start = prepend.start(), end = prepend.end(), "Prepend");
        info!(start = append.start(), end = append.end(), "Append");
        info!(start = new_range.start(), end = new_range.end(), "New range");

        let prepend_blocks = self.fetcher.get_blocks_range(prepend).await?;
        let append_blocks = self.fetcher.get_blocks_range(append).await?;
        let old_leftmost_block = self.fetcher.get_block(old_range.start().into()).await?;

        trie.prepend(prepend_blocks.iter(), &old_leftmost_block)?;
        trie.append(append_blocks.iter())?;

        let input = Input::AppendPrepend {
            elf_id: self.elf.id,
            prepend_blocks,
            append_blocks,
            old_leftmost_block,
            prev_zk_proof: Box::new((*old_zk_proof).clone()),
            block_trie: old_trie.clone(),
        };
        let receipt = self.prover.prove(&input, Some(old_zk_proof))?;
        let chain_update = ChainUpdate::from_two_tries(new_range, &old_trie, &trie, &receipt)?;

        Ok(Some(chain_update))
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use alloy_primitives::BlockNumber;
    use chain_test_utils::mock_provider;
    use ethers::providers::{MockProvider, Provider};
    use guest_wrapper::{CHAIN_GUEST_ELF, CHAIN_GUEST_IDS};
    use host_utils::ProofMode;
    use lazy_static::lazy_static;
    use risc0_zkvm::sha::Digest;

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

    #[cfg(test)]
    #[ctor::ctor]
    fn before_all() {
        unsafe {
            std::env::set_var("RISC0_DEV_MODE", "1");
        }
    }

    fn test_db() -> ChainDb {
        ChainDb::in_memory(
            // Current chain guest ELF ID is **not** included in `CHAIN_GUEST_IDS` when running without
            // `USE_DOCKER=1`. Therefore we need the .chain() call to add it.
            CHAIN_GUEST_IDS
                .iter()
                .map(|bytes| Digest::from_bytes(*bytes))
                .chain(iter::once(CHAIN_GUEST_ELF.id)),
        )
    }

    fn create_host(db: ChainDb, provider: Provider<MockProvider>) -> Host<MockProvider> {
        Host::from_parts(
            Prover::try_new(ProofMode::Fake, CHAIN_GUEST_ELF.clone()).unwrap(),
            BlockFetcher::from_provider(provider),
            db,
            1,
            CHAIN_GUEST_ELF.clone(),
            BlockTag::Latest,
            PREPEND_STRATEGY.clone(),
            APPEND_STRATEGY.clone(),
        )
    }

    #[tokio::test]
    async fn initialize() -> anyhow::Result<()> {
        let host = create_host(test_db(), mock_provider([LATEST], None));

        let chain_update = host.initialize().await?;
        let Host { mut db, .. } = host;
        db.update_chain(1, chain_update)?;

        let chain_trie = db.get_chain_trie(1)?;
        assert_eq!(chain_trie.block_range, LATEST..=LATEST);

        Ok(())
    }

    mod append_prepend {
        use super::*;

        async fn db_after_initialize() -> Result<ChainDb, HostError> {
            let mut host = create_host(test_db(), mock_provider([GENESIS], None));

            host.poll_commit().await?;
            let Host { db, .. } = host;

            Ok(db)
        }

        #[tokio::test]
        async fn no_new_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
            let mut host =
                create_host(db_after_initialize().await?, mock_provider([GENESIS], Some(GENESIS)));

            host.poll_commit().await?;
            let Host { db, .. } = host;

            let chain_trie = db.get_chain_trie(1)?;
            assert_eq!(chain_trie.block_range, GENESIS..=GENESIS);

            Ok(())
        }

        #[tokio::test]
        async fn new_confirmed_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
            let latest = GENESIS + CONFIRMATIONS;
            let new_confirmed_block = latest - CONFIRMATIONS + 1;
            let mut host = create_host(
                db_after_initialize().await?,
                mock_provider([new_confirmed_block, GENESIS], Some(latest)),
            );

            host.poll_commit().await?;
            let Host { db, .. } = host;

            let chain_trie = db.get_chain_trie(1)?;
            assert_eq!(chain_trie.block_range, GENESIS..=new_confirmed_block);

            Ok(())
        }

        // Reorg
    }
}
