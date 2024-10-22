use std::env::var;

use chain_host::{Host, HostConfig, ProofMode};
use trace::init_tracing;

mod trace;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    let config = HostConfig {
        rpc_url: var("RPC_URL")?,
        chain_id: 1,
        proof_mode: ProofMode::Fake,
        db_path: "chain_db".to_string(),
    };
    let mut host = Host::try_new(config)?;
    loop {
        let chain_update = host.poll().await?;
        host.db.update_chain(1, chain_update)?;
    }
}
