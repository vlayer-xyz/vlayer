use crate::errors::CLIError;
use crate::misc::parse_toml::get_src_from_string;
use crate::misc::path::find_foundry_toml;
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
            let toml_path = find_foundry_toml()?;
            println!("Found foundry.toml! (path: {:?})", toml_path);
            let contents = std::fs::read_to_string(toml_path)?;
            let src = get_src_from_string(contents)?;
            println!("src = {}", src)
        }
    }
    Ok(())
}
