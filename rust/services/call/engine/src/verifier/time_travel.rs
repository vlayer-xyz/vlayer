use alloy_primitives::{B256, BlockHash, BlockNumber, ChainId};
use async_trait::async_trait;
use common::sealed_with_test_mock;
use derive_new::new;
use tracing::info;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] chain_common::verifier::Error),
    #[error("Chain client error: {0}")]
    ChainClient(#[from] chain_client::Error),
    #[error("Block not found in chain proof trie: {block_num}")]
    BlockNotFound { block_num: BlockNumber },
    #[error(
        "Block hash mismatch: block_num={block_num}, hash_in_input={hash_in_input}, proven_hash={proven_hash}"
    )]
    BlockHash {
        block_num: BlockNumber,
        hash_in_input: B256,
        proven_hash: B256,
    },
    #[error("Attempted time travel while chain service for chain: {chain_id} is not available")]
    ChainServiceNotAvailable { chain_id: ChainId },
}

pub type Result = std::result::Result<(), Error>;
sealed_with_test_mock!(async IVerifier (chain_id: ChainId, blocks: Vec<(BlockNumber, BlockHash)>) -> Result);

#[derive(new)]
pub struct Verifier<C: chain_client::Client, V: chain_common::verifier::IVerifier> {
    chain_client: Option<C>,
    chain_proof_verifier: V,
}

impl<C: chain_client::Client, V: chain_common::verifier::IVerifier> seal::Sealed
    for Verifier<C, V>
{
}
#[async_trait]
impl<C: chain_client::Client, V: chain_common::verifier::IVerifier> IVerifier for Verifier<C, V> {
    async fn verify(&self, chain_id: ChainId, blocks: Vec<(BlockNumber, BlockHash)>) -> Result {
        info!("Verifying time-travel for chain_id: {chain_id}, blocks: {blocks:?}");
        if blocks.len() == 1 {
            info!("Single block, no need to verify chain proof");
            return Ok(());
        }
        let Some(ref client) = self.chain_client else {
            return Err(Error::ChainServiceNotAvailable { chain_id });
        };
        let block_numbers = blocks.iter().map(|(block_num, _)| *block_num).collect();
        let chain_proof = client.get_chain_proof(chain_id, block_numbers).await?;
        self.chain_proof_verifier.verify(chain_proof.as_ref())?;
        for (block_num, block_hash) in blocks {
            let trie_block_hash = chain_proof
                .block_trie
                .get(block_num)
                .ok_or(Error::BlockNotFound { block_num })?;
            if trie_block_hash != block_hash {
                return Err(Error::BlockHash {
                    block_num,
                    hash_in_input: block_hash,
                    proven_hash: trie_block_hash,
                });
            }
        }
        Ok(())
    }
}
