use clap::{Parser, Subcommand};
use commands::{
    init::{InitArgs, run_init},
    jwt::{Args as JwtArgs, run as run_jwt},
    update::UpdateArgs,
    web_proof::{WebProofArgs, webproof_fetch},
};
use test_runner::{cli::TestArgs, set_risc0_dev_mode};
use tracing::{debug, error, info, level_filters::LevelFilter, trace, warn};
use tracing_subscriber::EnvFilter;
pub use version::version;

use crate::{
    commands::{test::run_test, update::run_update},
    errors::Result,
};

mod cli_wrappers;
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
    Update(UpdateArgs),
    WebProofFetch(WebProofArgs),
    #[command(hide = true)]
    TestLoggingConfiguration,
    Jwt(JwtArgs),
}

#[tokio::main]
async fn main() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

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
        Commands::Update(args) => run_update(args).await,
        Commands::WebProofFetch(args) => {
            webproof_fetch(args).await.map_err(derive_more::Into::into)
        }
        Commands::TestLoggingConfiguration => run_logging_test(),
        Commands::Jwt(args) => run_jwt(args).map_err(crate::errors::Error::Jwt),
    }
}

fn run_logging_test() -> Result<()> {
    println!("printed");
    error!("error");
    warn!("warn");
    info!("info");
    debug!("debug");
    trace!("trace");
    Ok(())
}
