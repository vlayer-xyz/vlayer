use crate::errors::CLIError;
use server::server::{serve, ServerConfig};
use tracing::info;

pub(crate) async fn run_serve(server_config: ServerConfig) -> Result<(), CLIError> {
    info!("Running vlayer serve...");
    serve(server_config).await?;
    Ok(())
}
