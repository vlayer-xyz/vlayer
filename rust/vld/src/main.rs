mod cli;
mod config;
use clap::Parser;
use cli::{Cli, Commands, InfraCommands, InfraServices};
fn main() {
    let vlayer_path = config::get_vlayer_path();
    println!("vlayer_path: {vlayer_path}");
    let cli = Cli::parse();

    match &cli.command {
        Commands::Examples { name } => {
            println!("Performing examples action: {name}");
        }
        Commands::Infra { command } => match &command {
            InfraCommands::Run { command } => {
                let service = &command;
                {
                    match service {
                        InfraServices::WebProof => {
                            let docker_path = format!("{vlayer_path}/docker/web-proof");
                            println!("Running docker compose in: {docker_path}");
                            std::process::Command::new("docker-compose")
                                .args(["-f", "docker-compose-release.yaml", "up"])
                                .current_dir(&docker_path)
                                .spawn()
                                .expect("Failed to start docker-compose");
                        }
                        _ => {
                            let service_name = match service {
                                InfraServices::ChainServer => "chain_server",
                                InfraServices::ChainWorker => "chain_worker",
                                InfraServices::Vlayer => "vlayer",
                                _ => unreachable!(),
                            };
                            let docker_path = format!("{vlayer_path}/docker/{service_name}");
                            println!("Running docker run --build in: {docker_path}");
                            std::process::Command::new("docker")
                                .args(["run", "--build", "-f", "Dockerfile.nightly", "."])
                                .current_dir(&docker_path)
                                .spawn()
                                .expect("Failed to start docker run");
                        }
                    }
                }
            }
        },
        Commands::Contracts { action } => {
            println!("Performing contracts action: {action}");
        }
        Commands::Version => {
            println!("Vlad CLI Tool v1.0.0");
        }
    }
}
