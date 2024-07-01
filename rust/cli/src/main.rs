use crate::errors::CLIError;
use crate::misc::path::find_foundry_root;
use clap::{Parser, Subcommand};
use server::server::serve;

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
    match run().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), CLIError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve => {
            println!("running 'vlayer serve'");
            serve().await?;
        }
        Commands::Init => {
            println!(
                "running 'vlayer init' from directory: {:?}",
                std::env::current_dir()?
            );
            let root = find_foundry_root()?;
            println!("foundry root: {:?}", root);
        }
    }
    Ok(())
}
