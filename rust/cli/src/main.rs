use crate::errors::CLIError;
use clap::{Parser, Subcommand};
use commands::init::init;
use server::server::{serve, Config};
use test_runner::cli::TestArgs;
use tracing::{error, info};

mod commands;
pub mod errors;
mod utils;

#[cfg(test)]
mod test_utils;

const VERSION_MESSAGE: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_BUILD_TIMESTAMP"),
    ")"
);

#[derive(Parser)]
#[command(name = "vlayer", version = VERSION_MESSAGE, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Serve,
    Test(TestArgs),
}

#[tokio::main]
async fn main() {
    // In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    match run().await {
        Ok(_) => (),
        Err(e) => {
            error!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), CLIError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve => {
            info!("Running vlayer serve...");
            let config = Config {
                url: "http://localhost:8545".to_string(),
                port: 3000,
            };
            serve(config).await?;
        }
        Commands::Init => {
            let cwd = std::env::current_dir()?;
            init(cwd).await?;
        }
        Commands::Test(cmd) => {
            info!("Running vlayer tests");
            cmd.run().await.unwrap();
        }
    }
    Ok(())
}
