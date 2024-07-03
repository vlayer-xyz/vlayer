use crate::errors::CLIError;
use crate::misc::init::find_src;
use clap::{Parser, Subcommand};
use server::server::serve;
use tracing::{error, info};

pub mod errors;
mod misc;

#[cfg(test)]
mod test_utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Serve,
}

#[tokio::main]
async fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
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

    match &cli.command {
        Commands::Serve => {
            info!("Running vlayer serve...");
            serve().await?;
        }
        Commands::Init => {
            info!(
                "Running vlayer init from directory: {:?}",
                std::env::current_dir()?
            );
            let src = find_src()?;
            info!("Foundry source path = {}", src);
        }
    }
    Ok(())
}
