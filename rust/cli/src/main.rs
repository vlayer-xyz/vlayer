use clap::{Parser, Subcommand};
use commands::{
    args::{InitArgs, ServeArgs},
    init::run_init,
    serve::run_serve,
    version::Version,
};
use test_runner::cli::TestArgs;
use tracing::error;
use tracing_subscriber::EnvFilter;

use crate::{
    commands::{test::run_test, update::run_update},
    errors::CLIError,
};

mod commands;
pub mod errors;
mod utils;

#[cfg(test)]
mod test_utils;

#[derive(Parser)]
#[command(name = "vlayer", version = Version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init(InitArgs),
    Serve(ServeArgs),
    Test(Box<TestArgs>),
    Update,
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info"));

    // In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt().with_env_filter(filter).init();

    match run().await {
        Ok(_) => (),
        Err(e) => {
            error!("Error: {}", e);
            std::process::exit(e.error_code());
        }
    }
}

async fn run() -> Result<(), CLIError> {
    match Cli::parse().command {
        Commands::Serve(args) => run_serve(args).await,
        Commands::Init(args) => run_init(args).await,
        Commands::Test(args) => run_test(args).await,
        Commands::Update => run_update().await,
    }
}
