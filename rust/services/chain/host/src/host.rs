pub mod config;
pub mod error;

use alloy_primitives::ChainId;
use block_trie::BlockTrie;
use bytes::Bytes;
use chain_db::{ChainDb, ChainInfo, ChainUpdate};
use chain_guest::Input;
use chain_guest_wrapper::{RISC0_CHAIN_GUEST_ELF, RISC0_CHAIN_GUEST_ID};
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
use risc0_zkvm::{sha::Digest, ExecutorEnv, ProveInfo};
use serde::Serialize;

lazy_static! {
    static ref EMPTY_PROOF: Bytes = Bytes::new();
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
            Some(chain_info) => self.append_prepend(chain_info).await,
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
        let executor_env = build_executor_env(input)?;
        let ProveInfo { receipt, .. } =
            provably_execute(&self.prover, executor_env, RISC0_CHAIN_GUEST_ELF)?;

        let receipt_bytes: Bytes = bincode::serialize(&receipt)
            .expect("failed to serialize receipt")
            .into();
        let range = latest_block_number..=latest_block_number;
        let chain_info = ChainInfo::new(range, block_trie.hash_slow(), receipt_bytes);
        let chain_update = ChainUpdate::new(chain_info, &block_trie, []);
        Ok(chain_update)
    }

    async fn append_prepend(
        &self,
        current_chain_info: ChainInfo,
    ) -> Result<ChainUpdate, HostError> {
        let _latest_block = self.get_block(BlockTag::Latest).await?;
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
        use chain_db::InMemoryDatabase;
        use serde_json::Value;
        use test_utils::mock_provider;

        use super::*;

        const LATEST: u64 = 20_000_000;

        fn test_db() -> ChainDb {
            ChainDb::from_db(InMemoryDatabase::new())
        }

        mod poll {
            use alloy_primitives::B256;
            use mpt::MerkleTrie;
            use risc0_zkvm::Receipt;

            use super::*;

            fn assert_trie_proof(
                proof: Bytes,
                nodes: impl IntoIterator<Item = Bytes>,
            ) -> anyhow::Result<BlockTrie> {
                let receipt: Receipt = bincode::deserialize(&proof)?;
                receipt.verify(*GUEST_ID)?;

                let (proven_root, elf_id): (B256, Digest) = receipt.journal.decode()?;
                assert_eq!(elf_id, *GUEST_ID);
                let merkle_trie = MerkleTrie::from_rlp_nodes(nodes)?;
                assert_eq!(merkle_trie.hash_slow(), proven_root);

                let block_trie = BlockTrie::from(merkle_trie);
                Ok(block_trie)
            }

            #[tokio::test]
            async fn initialize() -> anyhow::Result<()> {
                let host =
                    Host::from_parts(Prover::default(), mock_provider([LATEST]), test_db(), 1);

                let ChainUpdate {
                    chain_info:
                        ChainInfo {
                            first_block,
                            last_block,
                            root_hash,
                            zk_proof,
                        },
                    added_nodes,
                    removed_nodes,
                } = host.poll().await?;

                host.provider.as_ref().assert_request(
                    "eth_getBlockByNumber",
                    Value::Array(vec!["latest".into(), false.into()]),
                )?;

                assert_eq!(first_block..=last_block, LATEST..=LATEST);

                let block_trie = assert_trie_proof(zk_proof, added_nodes)?;
                assert_eq!(block_trie.hash_slow(), root_hash);

                block_trie.get(LATEST).expect("block not found");
                assert!(removed_nodes.is_empty());

                Ok(())
            }

            mod append_prepend {
                use alloy_primitives::{BlockNumber, B256};

                use super::*;
                const GENERIS: BlockNumber = 0;

                #[tokio::test]
                async fn no_new_head_blocks_back_propagation_finished() -> anyhow::Result<()> {
                    let mut db = test_db();
                    let chain_info =
                        ChainInfo::new(GENERIS..=LATEST, B256::ZERO, EMPTY_PROOF.clone());
                    let chain_update = ChainUpdate::new(chain_info.clone(), [], []);
                    db.update_chain(1, chain_update)?;
                    let host = Host::from_parts(Prover::default(), mock_provider([LATEST]), db, 1);

                    let chain_update = host.poll().await?;

                    assert_eq!(chain_update, ChainUpdate::new(chain_info, [], []));

                    Ok(())
                }
                // No new head blocks, back propagation in progress
                // New head blocks, back propagation finished
                // New head blocks, back propagation in progress
                // Too many new head blocks
                // Reorg
            }
        }
    }
}
