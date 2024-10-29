use call_server::{serve, ProofMode};
use chain_server::server::{ChainProofServerMock, EMPTY_PROOF_RESPONSE};
use serde_json::json;
use tracing::{info, warn};

use crate::{commands::args::ServeArgs, errors::CLIError};

async fn start_chain_proof_server() -> ChainProofServerMock {
    ChainProofServerMock::start(json!({}), EMPTY_PROOF_RESPONSE.clone()).await
}

pub(crate) async fn run_serve(serve_args: ServeArgs) -> Result<(), CLIError> {
    let chain_proof_server_mock = start_chain_proof_server().await;
    let server_config = serve_args.into_server_config(&chain_proof_server_mock.url());

    info!("Running vlayer serve...");
    if let ProofMode::Fake = server_config.proof_mode {
        warn!("Running in fake mode. Server will not generate real proofs.");
    }
    serve(server_config).await?;
    Ok(())
}
