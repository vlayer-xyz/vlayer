use call_server::{ProofMode, ServerConfig};
use clap::{Parser, Subcommand};
use commands::{
    args::{InitArgs, ServeArgs},
    init::init,
    serve::run_serve,
    version::Version,
};
use test_runner::cli::TestArgs;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use crate::errors::CLIError;

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

#[derive(Subcommand)]
enum Commands {
    Init(InitArgs),
    Serve(ServeArgs),
    Test(TestArgs),
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
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), CLIError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve(serve_args) => {
            let proof_mode: ProofMode = serve_args.proof.unwrap_or_default().map();
            let server_config: ServerConfig =
                ServerConfig::new(serve_args.rpc_url, proof_mode, serve_args.host, serve_args.port);
            run_serve(server_config).await?;
        }
        Commands::Init(init_args) => {
            let existing = init_args.existing;
            let project_name = init_args.project_name;
            let template = init_args.template.unwrap_or_default();

            let cwd = std::env::current_dir()?;
            init(cwd, template, existing, project_name).await?;
        }
        Commands::Test(cmd) => {
            info!("Running vlayer tests");
            cmd.run().await.unwrap();
        }
    }
    Ok(())
}
