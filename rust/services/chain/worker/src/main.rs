use std::env::var;

use chain_host::{Host, HostConfig, ProofMode, Strategy};
use dotenvy::dotenv;
use trace::init_tracing;

mod trace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    init_tracing();
    let config = HostConfig {
        rpc_url: var("RPC_URL")?,
        chain_id: 1,
        proof_mode: ProofMode::Fake,
        db_path: "chain_db".to_string(),
        strategy: Strategy::new(
            parse_env_u64("MAX_HEAD_BLOCKS")?,
            parse_env_u64("MAX_BACK_PROPAGATION_BLOCKS")?,
            parse_env_u64("CONFIRMATIONS")?,
        ),
    };

    let mut host = Host::try_new(config)?;
    loop {
        host.poll_commit().await?;
    }
}

fn parse_env_u64(key: &str) -> anyhow::Result<u64> {
    var(key)?.parse().map_err(Into::into)
}
