use alloy_primitives::{BlockNumber, B256};
use async_trait::async_trait;
use static_assertions::assert_obj_safe;

use super::{ChainProofError, ChainProofVerifier};
use crate::evm::input::MultiEvmInput;

#[derive(thiserror::Error, Debug)]
pub enum GuestInputError {
    #[error("Chain proof error: {0}")]
    ChainProof(#[from] ChainProofError),
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

mod seal {

    // This trait prevents adding new implementations of GuestInputVerifier
    pub trait Sealed {}

    // Useful to mock verifier in tests
    #[cfg(feature = "testing")]
    impl<
            F: Fn(&super::MultiEvmInput) -> Result<(), super::GuestInputError> + Send + Sync + 'static,
        > Sealed for F
    {
    }
}

#[async_trait]
pub trait GuestInputVerifier: seal::Sealed + Send + Sync + 'static {
    async fn verify(&self, input: &MultiEvmInput) -> Result<(), GuestInputError>;
}

assert_obj_safe!(GuestInputVerifier);

// Useful to mock verifier in tests
// [auto_impl(Fn)] doesn't work with async_trait
#[cfg(feature = "testing")]
#[async_trait]
impl<F: Fn(&MultiEvmInput) -> Result<(), GuestInputError> + Send + Sync + 'static>
    GuestInputVerifier for F
{
    async fn verify(&self, input: &MultiEvmInput) -> Result<(), GuestInputError> {
        self(input)
    }
}

pub struct ZkGuestInputVerifier {
    chain_client: Box<dyn chain_client::Client>,
    verifier: Box<dyn ChainProofVerifier>,
}

impl ZkGuestInputVerifier {
    pub fn new(chain_client: impl chain_client::Client, verifier: impl ChainProofVerifier) -> Self {
        Self {
            chain_client: Box::new(chain_client),
            verifier: Box::new(verifier),
        }
    }
}

impl seal::Sealed for ZkGuestInputVerifier {}
#[async_trait]
impl GuestInputVerifier for ZkGuestInputVerifier {
    async fn verify(&self, input: &MultiEvmInput) -> Result<(), GuestInputError> {
        for (chain_id, blocks) in input.blocks_by_chain() {
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
                    .ok_or_else(|| GuestInputError::BlockNotFound { block_num })?;
                if trie_block_hash != block_hash {
                    return Err(GuestInputError::BlockHash {
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
