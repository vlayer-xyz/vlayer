use crate::errors::CLIError;
use server::server::{serve, Config};
use tracing::info;

pub(crate) async fn run_serve() -> Result<(), CLIError> {
    info!("Running vlayer serve...");
    let config: Config = Config::default();
    serve(config).await?;
    Ok(())
}
