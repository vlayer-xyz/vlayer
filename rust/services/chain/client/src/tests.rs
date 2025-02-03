use chain_common::mock_chain_proof;

use super::*;

fn get_cached_client(
    blocks_by_chain: impl IntoIterator<Item = (ChainId, Vec<BlockNumber>)>,
) -> CachedClient {
    let cache = blocks_by_chain
        .into_iter()
        .map(|(chain_id, block_numbers)| {
            let proof = mock_chain_proof(block_numbers.clone());
            (chain_id, (block_numbers, proof))
        })
        .collect();
    CachedClient::new(cache)
}

fn assert_cache_miss(err: Error, exp_chain_id: ChainId, exp_block_nums: &[BlockNumber]) {
    assert!(matches!(
        err,
        Error::CacheMiss {
            chain_id,
            block_numbers
        } if chain_id == exp_chain_id && block_numbers == exp_block_nums
    ));
}

mod cached_client {
    use super::*;

    #[tokio::test]
    async fn empty_cache() {
        let client = get_cached_client([]);
        let res = client.get_chain_proof(1, vec![1]).await;
        assert_cache_miss(res.unwrap_err(), 1, &[1]);
    }

    #[tokio::test]
    async fn cache_miss() {
        let client = get_cached_client([(1, vec![1])]);
        let res = client.get_chain_proof(2, vec![1]).await;
        assert_cache_miss(res.unwrap_err(), 2, &[1]);
    }

    #[tokio::test]
    async fn cache_hit() -> anyhow::Result<()> {
        let client = get_cached_client([(1, vec![1])]);
        let proof = client.get_chain_proof(1, vec![1]).await?;
        assert_eq!(proof, mock_chain_proof([1]));
        Ok(())
    }
}

mod caching_client {
    use super::*;

    #[tokio::test]
    async fn calls_cached() -> anyhow::Result<()> {
        let mock_client = get_cached_client([(1, vec![1])]);
        let client = RecordingClient::new(Box::new(mock_client));
        let proof = client.get_chain_proof(1, vec![1]).await?;
        assert_eq!(proof, mock_chain_proof([1]));
        let mut cache = client.into_cache();
        assert_eq!(cache.len(), 1);
        assert_eq!(cache.remove(&1).unwrap(), (vec![1], mock_chain_proof([1])));
        Ok(())
    }
}
