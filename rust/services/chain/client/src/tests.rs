use alloy_primitives::BlockHash;
use block_trie::BlockTrie;

use super::*;

fn mock_chain_proof(block_numbers: &[BlockNumber]) -> ChainProof {
    let mut block_trie = BlockTrie::default();
    for block_num in block_numbers {
        block_trie
            .insert_unchecked(*block_num, &BlockHash::default())
            .expect("insert_unchecked failed");
    }
    ChainProof {
        block_trie,
        proof: Default::default(),
    }
}

fn get_cached_client(
    blocks_by_chain: impl IntoIterator<Item = (ChainId, Vec<BlockNumber>)>,
) -> CachedChainProofClient {
    let cache = blocks_by_chain
        .into_iter()
        .map(|(chain_id, block_numbers)| {
            let proof = mock_chain_proof(&block_numbers);
            (chain_id, (block_numbers, proof))
        })
        .collect();
    CachedChainProofClient::new(cache)
}

fn assert_cache_miss(
    err: ChainProofClientError,
    exp_chain_id: ChainId,
    exp_block_nums: &[BlockNumber],
) {
    match err {
        ChainProofClientError::CacheMiss {
            chain_id,
            block_numbers,
        } => {
            assert_eq!(chain_id, exp_chain_id);
            assert_eq!(block_numbers, exp_block_nums);
        }
        err => panic!("Unexpected error: {err:?}"),
    }
}

mod cached_client {
    use super::*;

    #[tokio::test]
    async fn empty_cache() {
        let client = get_cached_client([]);
        let res = client.fetch_chain_proof(1, vec![1]).await;
        assert_cache_miss(res.unwrap_err(), 1, &[1]);
    }

    #[tokio::test]
    async fn cache_miss() {
        let client = get_cached_client([(1, vec![1])]);
        let res = client.fetch_chain_proof(2, vec![1]).await;
        assert_cache_miss(res.unwrap_err(), 2, &[1]);
    }

    #[tokio::test]
    async fn cache_hit() -> anyhow::Result<()> {
        let client = get_cached_client([(1, vec![1])]);
        let proof = client.fetch_chain_proof(1, vec![1]).await?;
        assert_eq!(proof, mock_chain_proof(&[1]));
        Ok(())
    }
}

mod caching_client {
    use super::*;

    #[tokio::test]
    async fn calls_cached() -> anyhow::Result<()> {
        let mock_client = get_cached_client([(1, vec![1])]);
        let client = CachingChainProofClient::new(mock_client);
        let proof = client.fetch_chain_proof(1, vec![1]).await?;
        assert_eq!(proof, mock_chain_proof(&[1]));
        let mut cache = client.into_cache();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.remove(&1).unwrap(), (vec![1], mock_chain_proof(&[1])));
        Ok(())
    }
}

mod get_chain_proofs {
    use super::*;

    #[tokio::test]
    async fn fetch_failed() {
        let client = get_cached_client([]);
        let res = client.get_chain_proofs(HashMap::from([(1, vec![1])])).await;
        assert!(res.is_err())
    }

    #[tokio::test]
    async fn ok() -> anyhow::Result<()> {
        let blocks_by_chain = [(1, vec![1]), (2, vec![1, 2])];
        let client = get_cached_client(blocks_by_chain.clone());
        let mut proof = client.get_chain_proofs(blocks_by_chain.into()).await?;
        assert_eq!(proof.len(), 2);
        assert_eq!(proof.remove(&1).unwrap(), mock_chain_proof(&[1]));
        assert_eq!(proof.remove(&2).unwrap(), mock_chain_proof(&[1, 2]));
        Ok(())
    }
}
