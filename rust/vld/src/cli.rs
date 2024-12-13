use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vlad")]
#[command(about = "Vlayer internal CLI ", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Examples {
        #[command(subcommand)]
        command: ExampleCommands,
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
    Stop {
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

#[derive(Subcommand)]
pub enum ExampleCommands {
    Run {
        #[command(subcommand)]
        command: ExampleServices,
    },
}

#[derive(Subcommand)]
pub enum ExampleServices {
    WebProof,
}
