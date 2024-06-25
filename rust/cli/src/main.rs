use clap::{Parser, Subcommand};
use server::app::server;
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
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve => {
            println!("running 'vlayer serve'");
            server().await
        }
        Commands::Init => {
            println!("running 'vlayer init'");
            Ok(())
        }
    }
}
