use call_server::serve;
use mock_chain_server::{ChainProofServerMock, EMPTY_PROOF_RESPONSE};
use serde_json::json;
use tracing::{info, warn};

use crate::{
    commands::{args::ServeArgs, version::version},
    errors::CLIError,
};

async fn start_chain_proof_server() -> ChainProofServerMock {
    ChainProofServerMock::start(json!({}), EMPTY_PROOF_RESPONSE.clone()).await
}

pub(crate) async fn run_serve(serve_args: ServeArgs) -> Result<(), CLIError> {
    let chain_proof_server_mock = start_chain_proof_server().await;
    let semver = version();
    let server_config = serve_args.into_server_config(chain_proof_server_mock.url(), semver);

    info!("Running vlayer serve...");
    if server_config.fake_proofs() {
        warn!("Running in fake mode. Server will not generate real proofs.");
    }
    serve(server_config).await?;
    Ok(())
}
