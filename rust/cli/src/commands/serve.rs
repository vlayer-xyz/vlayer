use crate::errors::CLIError;
use call_server::{serve, ProofMode, ServerConfig};
use tracing::{info, warn};

pub(crate) async fn run_serve(server_config: ServerConfig) -> Result<(), CLIError> {
    info!("Running vlayer serve...");
    if let ProofMode::Fake = server_config.proof_mode {
        warn!("Running in fake mode. Server will not generate real proofs.");
    }
    serve(server_config).await?;
    Ok(())
}
