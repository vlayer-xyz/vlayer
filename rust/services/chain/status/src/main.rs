use std::{collections::BTreeMap, time::Duration};

use chain::{CHAIN_ID_TO_CHAIN_SPEC, ChainSpec};
use chain_client::{ChainClientConfig, Client, Error, RpcClient};
use server_utils::rpc::Error as RpcError;
use tokio::time::sleep;
use u64_range::NonEmptyRange;

const TEST_URL: &str = "https://test-chainservice.vlayer.xyz";
const PROD_URL: &str = "https://chainservice.vlayer.xyz";

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let test_client = create_client(TEST_URL);
    let prod_client = create_client(PROD_URL);
    println!("================ Test Client URL: {TEST_URL}");
    check_server_sync_status(&test_client).await;
    println!("================ Prod Client URL: {PROD_URL}");
    check_server_sync_status(&prod_client).await;

    Ok(())
}

fn create_client(url: &str) -> RpcClient {
    let config = ChainClientConfig {
        url: url.to_string(),
        poll_interval: Duration::from_secs(0),
        timeout: Duration::from_secs(0),
    };
    RpcClient::new(&config)
}

async fn check_server_sync_status(client: &RpcClient) {
    let chains_ordered_by_id: BTreeMap<_, _> = CHAIN_ID_TO_CHAIN_SPEC.iter().collect();

    for (_, chain) in chains_ordered_by_id {
        print_sync_status(client, chain).await;
        sleep(Duration::from_secs(1)).await;
    }
}

async fn print_sync_status(client: &RpcClient, chain: &ChainSpec) {
    match client.get_sync_status(chain.id()).await {
        Ok(range) => {
            #[allow(clippy::expect_used)]
            let range = NonEmptyRange::try_from_range(range.first_block..=range.last_block)
                .expect("Range is non-empty");
            println!("✅ Chain: {} {}, Range: {}", chain.id(), chain.name(), range);
        }
        Err(Error::Rpc(RpcError::JsonRpc(value))) => {
            println!("❌ Rpc error {}: {}", chain.name(), value);
        }
        Err(Error::Rpc(RpcError::Http(err))) => {
            println!("❌ Http error {}: {}", chain.name(), err);
        }
        Err(err) => {
            println!("❌ Unknown error fetching sync status for {}: {}", chain.name(), err);
        }
    }
}
