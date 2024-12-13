use clap::{Parser, Subcommand};

/// Vlad CLI Tool
#[derive(Parser)]
#[command(name = "vlad")]
#[command(about = "Vlayer internal CLI ", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
enum InfraCommands {
    Run {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let vlayer_path = get_vlayer_path();
    println!("vlayer_path: {}", vlayer_path);
    let cli = Cli::parse();

    match &cli.command {
        Commands::Examples { name } => {
            println!("Performing examples action: {}", name);
        }
        Commands::Infra { command } => {
            match &command {
                InfraCommands::Run { name } => {
                    println!("Performing infra action: {}", name);
                }
            }
        }
        Commands::Contracts { action } => {
            println!("Performing contracts action: {}", action);
        }
        Commands::Version => {
            println!("Vlad CLI Tool v1.0.0");
        }
    }
}


fn get_vlayer_path() -> String {
    let home_dir = std::env::var("HOME").expect("Failed to get home directory");
    let vld_path = std::path::Path::new(&home_dir).join(".vld");
    let content = std::fs::read_to_string(vld_path)
        .expect("Failed to read ~/.vld");
    
    let vlayer_path = content
        .lines()
        .find(|line| line.starts_with("VLAYER_PATH="))
        .map(|line| line.trim_start_matches("VLAYER_PATH=").trim().to_string())
        .expect("Could not find VLAYER_PATH in ~/.vld");

    if vlayer_path.is_empty() {
        panic!("VLAYER_PATH value in ~/.vld is empty");
    }
    vlayer_path
}

