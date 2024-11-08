use alloy_primitives::ChainId;
use chain_client::CachedClient;
use chain_common::ChainProof;
use risc0_zkp::verify::VerificationError;

use super::*;
use crate::evm::{
    env::location::ExecutionLocation,
    input::{EvmInput, MultiEvmInput},
};

const CHAIN_ID: ChainId = 1;
const INVALID_BLOCK_HASH: B256 = B256::ZERO;

fn mock_chain_client(
    cache: Vec<(ChainId, (Vec<BlockNumber>, ChainProof))>,
) -> impl chain_client::Client {
    let cache = cache.into_iter().collect();
    CachedClient::new(cache)
}

fn mock_chain_proof(block_trie: BlockTrie) -> ChainProof {
    ChainProof {
        block_trie,
        ..Default::default()
    }
}

fn mock_evm_input(header: Box<dyn EvmBlockHeader>) -> EvmInput {
    EvmInput {
        header,
        state_trie: Default::default(),
        storage_tries: Default::default(),
        contracts: Default::default(),
        ancestors: Default::default(),
    }
}

fn mock_multi_evm_input(blocks: RangeInclusive<BlockNumber>) -> MultiEvmInput {
    let headers = mock_block_headers(blocks);
    let inputs = headers
        .into_iter()
        .map(|header| (ExecutionLocation::new(header.number(), CHAIN_ID), mock_evm_input(header)))
        .collect();
    MultiEvmInput { inputs }
}

const fn proof_ok(_: &ChainProof) -> Result<(), ChainProofError> {
    Ok(())
}

const fn proof_invalid(_: &ChainProof) -> Result<(), ChainProofError> {
    Err(ChainProofError::Zk(VerificationError::InvalidProof))
}

async fn verify_guest_input(
    chain_client: impl chain_client::Client,
    verifier: impl ChainProofVerifier,
    input: &MultiEvmInput,
) -> Result<(), GuestInputError> {
    ZkGuestInputVerifier::new(chain_client, verifier)
        .verify(input)
        .await
}

#[tokio::test]
async fn ok() {
    let block_trie = mock_block_trie(0..=0);
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
    let input = mock_multi_evm_input(0..=0);

    verify_guest_input(chain_client, proof_ok, &input)
        .await
        .expect("verfification should succeed");
}

#[tokio::test]
async fn chain_proof_missing() {
    let chain_client = mock_chain_client(vec![]);
    let input = mock_multi_evm_input(0..=0);

    let res = verify_guest_input(chain_client, proof_ok, &input).await;
    assert!(matches!(res.unwrap_err(), GuestInputError::ChainClient(..)));
}

#[tokio::test]
async fn chain_proof_invalid() {
    let block_trie = mock_block_trie(0..=0);
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
    let input = mock_multi_evm_input(0..=0);

    let res = verify_guest_input(chain_client, proof_invalid, &input).await;
    assert!(matches!(res.unwrap_err(), GuestInputError::ChainProof(..)));
}

#[tokio::test]
async fn block_not_in_trie() {
    let chain_proof = mock_chain_proof(BlockTrie::default());
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
    let input = mock_multi_evm_input(0..=0);

    let res = verify_guest_input(chain_client, proof_ok, &input).await;
    assert!(matches!(res.unwrap_err(), GuestInputError::BlockNotFound { block_num: 0 }));
}

#[tokio::test]
async fn block_hash_mismatch() {
    let block_header = mock_block_header(0, Default::default());
    let _block_hash = block_header.hash_slow();
    let mut block_trie = BlockTrie::default();
    block_trie.insert_unchecked(0, &INVALID_BLOCK_HASH).unwrap();
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0], chain_proof))]);
    let input = mock_multi_evm_input(0..=0);

    let res = verify_guest_input(chain_client, proof_ok, &input).await;
    assert!(matches!(
        res.unwrap_err(),
        GuestInputError::BlockHash {
            block_num: 0,
            hash_in_input: _block_hash,
            proven_hash: INVALID_BLOCK_HASH
        }
    ));
}
