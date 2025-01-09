use std::time::Duration;

use call_host::{AwaitingChainProofError, Host};
use derive_new::new;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Waiting for chain proof timed out")]
    Timeout,
    #[error(transparent)]
    AwaitingChainProof(#[from] AwaitingChainProofError),
}

pub async fn await_ready(host: &Host, config: Option<Config>) -> Result<(), Error> {
    if let Some(Config {
        poll_interval,
        timeout,
        ..
    }) = config
    {
        // Wait for chain proof if necessary
        let start = tokio::time::Instant::now();
        while !host.chain_proof_ready().await? {
            info!(
                "Location {:?} not indexed. Waiting for chain proof",
                host.start_execution_location()
            );
            tokio::time::sleep(poll_interval).await;
            if start.elapsed() > timeout {
                return Err(Error::Timeout);
            }
        }
    }
    Ok(())
}

#[derive(new, Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub url: String,
    pub poll_interval: Duration,
    pub timeout: Duration,
}
