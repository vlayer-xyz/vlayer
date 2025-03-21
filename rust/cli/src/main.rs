use clap::{Parser, Subcommand};
use commands::{
    init::{run_init, InitArgs},
    web_proof::{webproof_fetch, WebProof, WebProofArgs},
};
use test_runner::{cli::TestArgs, set_risc0_dev_mode};
use tracing::error;
use tracing_subscriber::EnvFilter;
pub use version::version;

pub use crate::{
    commands::{test::run_test, update::run_update},
    errors::Result,
};

mod commands;
mod config;
pub mod errors;
mod soldeer;
mod utils;

#[cfg(test)]
mod test_utils;

#[derive(Parser)]
#[command(name = "vlayer", version = version(), about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init(InitArgs),
    Test(Box<TestArgs>),
    Update,
    #[command(hide = true)]
    WebProofFetch(WebProofArgs),
}

#[tokio::main]
async fn main() {
    // In order to view logs, run `RUST_LOG=info cargo run`
    let filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Err(e) = Box::pin(run()).await {
        error!("Error: {:#}", e);
        std::process::exit(e.error_code());
    }
}

async fn run() -> Result<()> {
    match Cli::parse().command {
        Commands::Init(args) => run_init(args).await,
        Commands::Test(args) => {
            set_risc0_dev_mode();
            Box::pin(run_test(args)).await
        }
        Commands::Update => run_update().await,
        Commands::WebProofFetch(args) => webproof_fetch(args, WebProof::new()).await,
    }
}
