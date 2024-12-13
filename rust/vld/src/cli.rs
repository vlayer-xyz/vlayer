use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vlad")]
#[command(about = "Vlayer internal CLI ", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[cfg(not(clippy))]
#[allow(dead_code)]
mod private {
    include!(concat!(env!("OUT_DIR"), "/directories.rs"));
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    #[clap(alias = "e")]
    Examples {
        #[command(subcommand)]
        command: ExampleCommands,
    },
    #[clap(alias = "r")]
    Rust {
        #[command(subcommand)]
        dir: private::Rust,
    },
    Js {
        #[command(subcommand)]
        dir: private::JS,
    },
    #[clap(alias = "c")]
    Contracts {
        #[arg(short, long)]
        action: String,
    },
    #[clap(alias = "i")]
    Infra {
        #[command(subcommand)]
        command: InfraCommands,
    },
    #[clap(alias = "v")]
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
