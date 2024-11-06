use alloy_primitives::{BlockNumber, B256};

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

pub async fn verify_guest_input(
    chain_client: impl chain_client::Client,
    verifier: impl ChainProofVerifier,
    input: &MultiEvmInput,
) -> Result<(), GuestInputError> {
    for (chain_id, blocks) in input.blocks_by_chain() {
        let block_numbers = blocks.iter().map(|(block_num, _)| *block_num).collect();
        let chain_proof = chain_client
            .get_chain_proof(chain_id, block_numbers)
            .await?;
        verifier.verify(&chain_proof)?;
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
