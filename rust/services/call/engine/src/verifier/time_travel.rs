use alloy_primitives::{BlockHash, BlockNumber, ChainId, B256};
use async_trait::async_trait;
use derive_new::new;
use static_assertions::assert_obj_safe;

use super::{chain_proof, define_sealed_trait};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] super::chain_proof::Error),
    #[error("Chain client error: {0}")]
    ChainClient(#[from] chain_client::Error),
    #[error("Block not found in chain proof trie: {block_num}")]
    BlockNotFound { block_num: BlockNumber },
    #[error("Block hash mismatch: block_num={block_num}, hash_in_input={hash_in_input}, proven_hash={proven_hash}")]
    BlockHash {
        block_num: BlockNumber,
        hash_in_input: B256,
        proven_hash: B256,
    },
}

pub type Result = std::result::Result<(), Error>;

define_sealed_trait!(super::ChainId, Vec<(super::BlockNumber, super::BlockHash)>);

#[async_trait]
pub trait IVerifier: seal::Sealed + Send + Sync {
    async fn verify(&self, chain_id: ChainId, blocks: Vec<(BlockNumber, BlockHash)>) -> Result;
}

assert_obj_safe!(IVerifier);

// Useful to mock verifier in tests
// [auto_impl(Fn)] doesn't work with async_trait
#[cfg(test)]
#[async_trait]
impl<F: Fn(ChainId, Vec<(BlockNumber, BlockHash)>) -> Result + Send + Sync> IVerifier for F {
    async fn verify(&self, chain_id: ChainId, blocks: Vec<(BlockNumber, BlockHash)>) -> Result {
        self(chain_id, blocks)
    }
}

#[derive(new)]
pub struct Verifier<C: chain_client::Client, V: chain_proof::IVerifier> {
    chain_client: C,
    chain_proof_verifier: V,
}

impl<C: chain_client::Client, V: chain_proof::IVerifier> Verifier<C, V> {
    pub fn into_parts(self) -> (C, V) {
        (self.chain_client, self.chain_proof_verifier)
    }
}

impl<C: chain_client::Client, V: chain_proof::IVerifier> seal::Sealed for Verifier<C, V> {}
#[async_trait]
impl<C: chain_client::Client, V: chain_proof::IVerifier> IVerifier for Verifier<C, V> {
    async fn verify(&self, chain_id: ChainId, blocks: Vec<(BlockNumber, BlockHash)>) -> Result {
        if blocks.len() == 1 {
            return Ok(()); // No need to verify chain proofs for a single location
        }
        let block_numbers = blocks.iter().map(|(block_num, _)| *block_num).collect();
        let chain_proof = self
            .chain_client
            .get_chain_proof(chain_id, block_numbers)
            .await?;
        self.chain_proof_verifier.verify(&chain_proof)?;
        for (block_num, block_hash) in blocks {
            let trie_block_hash = chain_proof
                .block_trie
                .get(block_num)
                .ok_or_else(|| Error::BlockNotFound { block_num })?;
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
