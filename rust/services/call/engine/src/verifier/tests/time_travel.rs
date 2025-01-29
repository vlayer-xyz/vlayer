use alloy_primitives::{BlockHash, ChainId};
use block_trie::mock_block_trie;
use chain_client::CachedClient;
use chain_common::ChainProof;
use risc0_zkp::verify::VerificationError;

use super::*;
use crate::verifier::{
    chain_proof,
    time_travel::{Error, IVerifier, Verifier},
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

fn mock_time_travel_destinations(
    blocks: RangeInclusive<BlockNumber>,
) -> Vec<(BlockNumber, BlockHash)> {
    let headers = mock_block_headers(blocks);
    headers
        .into_iter()
        .map(|header| (header.number(), header.hash_slow()))
        .collect()
}

const fn proof_ok(_: &ChainProof) -> chain_proof::Result {
    Ok(())
}

const fn proof_invalid(_: &ChainProof) -> chain_proof::Result {
    Err(chain_proof::Error::Zk(VerificationError::InvalidProof))
}

async fn verify_time_travel_destinations(
    chain_client: impl chain_client::Client,
    verifier: impl chain_proof::IVerifier,
    destinations: Vec<(BlockNumber, BlockHash)>,
) -> Result<(), Error> {
    Verifier::new(Some(chain_client), verifier)
        .verify(CHAIN_ID, destinations)
        .await
}

#[tokio::test]
async fn ok() {
    let block_trie = mock_block_trie(0..=1);
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0, 1], chain_proof))]);
    let input = mock_time_travel_destinations(0..=1);

    verify_time_travel_destinations(chain_client, proof_ok, input)
        .await
        .expect("verification should succeed");
}

#[tokio::test]
async fn single_location_no_chain_proof() {
    let chain_client = mock_chain_client(vec![]);
    let input = mock_time_travel_destinations(0..=0);

    verify_time_travel_destinations(chain_client, proof_invalid, input)
        .await
        .expect("verification should succeed");
}

#[tokio::test]
async fn chain_proof_missing() {
    let chain_client = mock_chain_client(vec![]);
    let input = mock_time_travel_destinations(0..=1);

    let res = verify_time_travel_destinations(chain_client, proof_ok, input).await;
    assert!(matches!(res.unwrap_err(), Error::ChainClient(..)));
}

#[tokio::test]
async fn chain_proof_invalid() {
    let block_trie = mock_block_trie(0..=1);
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0, 1], chain_proof))]);
    let input = mock_time_travel_destinations(0..=1);

    let res = verify_time_travel_destinations(chain_client, proof_invalid, input).await;
    assert!(matches!(res.unwrap_err(), Error::ChainProof(..)));
}

#[tokio::test]
async fn block_not_in_trie() {
    let block_trie = mock_block_trie(0..=0);
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0, 1], chain_proof))]);
    let input = mock_time_travel_destinations(0..=1);

    let res = verify_time_travel_destinations(chain_client, proof_ok, input).await;
    assert!(matches!(res.unwrap_err(), Error::BlockNotFound { block_num: 1 }));
}

#[tokio::test]
async fn block_hash_mismatch() {
    let mut block_headers = mock_block_headers(0..=1);
    let _block_hash = block_headers[1].hash_slow();
    let mut block_trie = BlockTrie::init(block_headers.remove(0)).unwrap();
    block_trie.insert_unchecked(1, &INVALID_BLOCK_HASH).unwrap();
    let chain_proof = mock_chain_proof(block_trie);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0, 1], chain_proof))]);
    let input = mock_time_travel_destinations(0..=1);

    let res = verify_time_travel_destinations(chain_client, proof_ok, input).await;
    assert!(matches!(
        res.unwrap_err(),
        Error::BlockHash {
            block_num: 1,
            hash_in_input: _block_hash,
            proven_hash: INVALID_BLOCK_HASH
        }
    ));
}
