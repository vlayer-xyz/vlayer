use alloy_primitives::{BlockHash, ChainId};
use chain_client::CachedClient;
use chain_common::{
    ChainProof, ChainProofRef, mock_chain_proof_receipt, mock_chain_proof_with_hashes,
};
use risc0_zkp::verify::VerificationError;

use super::*;
use crate::verifier::time_travel::{Error, IVerifier, Verifier};

const CHAIN_ID: ChainId = 1;
const INVALID_BLOCK_HASH: B256 = B256::ZERO;

fn mock_chain_client(
    cache: Vec<(ChainId, (Vec<BlockNumber>, ChainProof))>,
) -> impl chain_client::Client {
    let cache = cache.into_iter().collect();
    CachedClient::new(cache)
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

const fn proof_ok(_: ChainProofRef) -> chain_common::verifier::Result {
    Ok(())
}

const fn proof_invalid(_: ChainProofRef) -> chain_common::verifier::Result {
    Err(chain_common::verifier::Error::Zk(VerificationError::InvalidProof))
}

async fn verify_time_travel_destinations(
    chain_client: impl chain_client::Client,
    verifier: impl chain_common::verifier::IVerifier,
    destinations: Vec<(BlockNumber, BlockHash)>,
) -> Result<(), Error> {
    Verifier::new(Some(chain_client), verifier)
        .verify(CHAIN_ID, destinations)
        .await
}

#[tokio::test]
async fn ok() {
    let chain_proof = mock_chain_proof_with_hashes(0..=1);
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
    let chain_proof = mock_chain_proof_with_hashes(0..=1);
    let chain_client = mock_chain_client(vec![(CHAIN_ID, (vec![0, 1], chain_proof))]);
    let input = mock_time_travel_destinations(0..=1);

    let res = verify_time_travel_destinations(chain_client, proof_invalid, input).await;
    assert!(matches!(res.unwrap_err(), Error::ChainProof(..)));
}

#[tokio::test]
async fn block_not_in_trie() {
    let chain_proof = mock_chain_proof_with_hashes(0..=0);
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
    let chain_proof = ChainProof {
        receipt: mock_chain_proof_receipt(),
        block_trie,
    };
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
