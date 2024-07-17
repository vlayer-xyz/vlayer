use crate::tests::TestArgs;
use crate::{errors::CLIError, misc::init::find_src_path};
use clap::{Parser, Subcommand};
use misc::{
    init::{create_vlayer_dir, fetch_vlayer_files},
    path::find_foundry_root,
};
use server::server::{serve, Config};
use tracing::{error, info};

pub mod errors;
mod misc;

#[cfg(test)]
mod test_utils;
mod tests;

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
            info!("Running vlayer init from directory {:?}", cwd.display());

            let root_path = find_foundry_root(&cwd)?;
            let src_path = find_src_path(&root_path)?;
            info!("Found foundry project root in \"{}\"", &src_path.display());

            match create_vlayer_dir(&src_path)? {
                true => {
                    info!("Created vlayer directory in \"{}\"", src_path.display());
                    fetch_vlayer_files(&src_path).await?
                }
                false => error!(
                    "vlayer directory already exists in \"{}\". Skipping creation.",
                    &src_path.display()
                ),
            }
        }
        Commands::Test(cmd) => {
            info!("Running vlayer tests");
            cmd.run().await.unwrap();
        }
    }
    Ok(())
}
