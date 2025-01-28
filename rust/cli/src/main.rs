use clap::{Parser, Subcommand};
use commands::{args::InitArgs, init::run_init};
use test_runner::cli::TestArgs;
use tracing::error;
use tracing_subscriber::EnvFilter;
pub use version::version as build_version;

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
#[command(name = "vlayer", version = build_version(), about, long_about = None)]
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
}

#[tokio::main]
async fn main() {
    // In order to view logs, run `RUST_LOG=info cargo run`
    let filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    if let Err(e) = Box::pin(run()).await {
        error!("Error: {}", e);
        std::process::exit(e.error_code());
    }
}

async fn run() -> Result<(), CLIError> {
    match Cli::parse().command {
        Commands::Init(args) => run_init(args).await,
        Commands::Test(args) => Box::pin(run_test(args)).await,
        Commands::Update => run_update(),
    }
}

pub(crate) fn target_version() -> String {
    if let Ok(version) = std::env::var("VLAYER_DEBUG_VERSION_OVERRIDE") {
        // When building CLI from source and trying to target released templates and contracts,
        // we cannot rely on the baked-in version, but instead we need to explicitly pass version of
        // some already published artifacts.
        // This is a temporary measure used only for internal debugging.
        return version;
    }
    build_version()
}
