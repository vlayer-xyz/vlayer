use clap::{Parser, Subcommand};

use crate::config;

#[derive(Parser)]
#[command(name = "vlad")]
#[command(about = "Vlayer internal CLI ", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Examples {
        #[arg(short, long)]
        name: String,
    },
    Contracts {
        #[arg(short, long)]
        action: String,
    },
    Infra {
        #[command(subcommand)]
        command: InfraCommands,
    },
    Version,
}

#[derive(Subcommand)]
pub enum InfraCommands {
    Run {
        #[command(subcommand)]
        command: InfraServices,
    },
}

#[derive(Subcommand)]
pub enum InfraServices {
    ChainServer,
    ChainWorker,
    WebProof,
    Vlayer,
}
