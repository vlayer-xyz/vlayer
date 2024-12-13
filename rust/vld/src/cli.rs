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

pub use private::Examples as PrivateExamples;

#[derive(Subcommand)]
pub enum Commands {
    Init,
    #[clap(alias = "e")]
    Examples {
        #[command(subcommand)]
        command: private::Examples,
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
        #[command(subcommand)]
        command: ContractCommands,
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

#[derive(Subcommand, Debug)]
pub enum ExampleCommands {
    Run,
}

#[derive(Subcommand)]
pub enum ContractCommands {
    Rebuild
}