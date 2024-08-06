use crate::errors::CLIError;
use server::server::{serve, Config};
use tracing::info;

const URL: &str = "http://localhost:8545";
const PORT: u16 = 3000;

pub(crate) async fn run_serve() -> Result<(), CLIError> {
    info!("Running vlayer serve...");
    let config = Config {
        url: URL.to_string(),
        port: PORT,
    };
    serve(config).await?;
    Ok(())
}
