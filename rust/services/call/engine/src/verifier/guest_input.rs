use alloy_primitives::{BlockNumber, B256};
use async_trait::async_trait;
use derive_new::new;
use static_assertions::assert_obj_safe;

use super::chain_proof;
use crate::evm::input::MultiEvmInput;

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

mod seal {

    // This trait prevents adding new implementations of Verifier
    pub trait Sealed {}

    // Useful to mock verifier in tests
    #[cfg(feature = "testing")]
    impl<F> Sealed for F where F: Fn(&super::MultiEvmInput) -> super::Result + Send + Sync {}
}

#[async_trait]
pub trait Verifier: seal::Sealed + Send + Sync {
    async fn verify(&self, input: &MultiEvmInput) -> Result;
}

assert_obj_safe!(Verifier);

// Useful to mock verifier in tests
// [auto_impl(Fn)] doesn't work with async_trait
#[cfg(feature = "testing")]
#[async_trait]
impl<F: Fn(&MultiEvmInput) -> Result + Send + Sync> Verifier for F {
    async fn verify(&self, input: &MultiEvmInput) -> Result {
        self(input)
    }
}

#[derive(new)]
pub struct ZkVerifier<C: chain_client::Client, V: chain_proof::Verifier> {
    chain_client: C,
    verifier: V,
}

impl<C: chain_client::Client, V: chain_proof::Verifier> ZkVerifier<C, V> {
    pub fn into_parts(self) -> (C, V) {
        (self.chain_client, self.verifier)
    }
}

impl<C: chain_client::Client, V: chain_proof::Verifier> seal::Sealed for ZkVerifier<C, V> {}
#[async_trait]
impl<C: chain_client::Client, V: chain_proof::Verifier> Verifier for ZkVerifier<C, V> {
    async fn verify(&self, input: &MultiEvmInput) -> Result {
        for (chain_id, blocks) in input.blocks_by_chain() {
            if blocks.len() == 1 {
                return Ok(()); // No need to verify chain proofs for a single location
            }
            let block_numbers = blocks.iter().map(|(block_num, _)| *block_num).collect();
            let chain_proof = self
                .chain_client
                .get_chain_proof(chain_id, block_numbers)
                .await?;
            self.verifier.verify(&chain_proof)?;
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
        }
        Ok(())
    }
}
