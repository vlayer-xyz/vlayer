mod cli;
mod commands;
mod config;

use std::{
    io::{self, Write},
    path::Path,
    process::Command,
};

use clap::Parser;
use cli::{Cli, Commands, ContractCommands, ExampleCommands, InfraCommands, InfraServices};

use crate::cli::PrivateExamples;

fn print_dir(path: &str, dir: &str) {
    let vlayer_path = config::get_vlayer_path();
    let path = Path::new(&vlayer_path).join(path).join(dir);
    println!("{}", path.to_str().unwrap());
}
fn main() {
    // Check all required ports
    let ports = [3000, 8545, 5173, 5175, 3011];
    for port in ports {
        if let Err(e) = check_and_kill_port(port) {
            eprintln!("Error checking port {port}: {e}");
            std::process::exit(1);
        }
    }
    let cli = Cli::parse();

    let mut child_processes: Vec<std::process::Child> = Vec::new();

    match &cli.command {
        Commands::Init => {
            commands::init::init();
        }
        Commands::Examples { command } => {
            let vlayer_path = config::get_vlayer_path();

            let path = &command.to_string();

            let command = match command {
                PrivateExamples::SimpleWebProof { command } => command,
                PrivateExamples::SimpleTimeTravel { command } => command,
                PrivateExamples::Simple { command } => command,
                PrivateExamples::SimpleEmailProof { command } => command,
                PrivateExamples::SimpleTeleport { command } => command,
            };
            match &command {
                Some(ExampleCommands::Run) => {
                    let web_proof_docker = run_web_proof_docker(&vlayer_path);
                    child_processes.push(web_proof_docker);

                    let mut attempts = 0;
                    while attempts < 30 {
                        if let Ok(socket) = std::net::TcpStream::connect("127.0.0.1:8545") {
                            drop(socket);
                            break;
                        }
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        attempts += 1;
                    }
                    if attempts == 30 {
                        eprintln!("Timeout waiting for anvil services to be ready");
                        std::process::exit(1);
                    }
                    println!("Anvil services are ready");
                    deploy_contracts(&vlayer_path);

                    let vlayer = run_vlayer_bash(&vlayer_path);
                    child_processes.push(vlayer);

                    let web_app = run_web_app(&vlayer_path);
                    child_processes.push(web_app);

                    let extension = run_browser_extension(&vlayer_path);
                    child_processes.push(extension);
                }
                None => {
                    print_dir("examples", path);
                }
            }
        }
        Commands::Rust { dir } => {
            print_dir("rust", &dir.to_string());
        }
        Commands::Js { dir } => {
            print_dir("packages", &dir.to_string());
        }
        Commands::Infra { command } => {
            let vlayer_path = config::get_vlayer_path();
            match &command {
                InfraCommands::Run { command } => {
                    let service = &command;
                    match service {
                        InfraServices::WebProof => {
                            let web_proof = run_web_proof_docker(&vlayer_path);
                            child_processes.push(web_proof);
                            let vlayer = run_vlayer_bash(&vlayer_path);
                            child_processes.push(vlayer);
                        }
                        InfraServices::Vlayer => {
                            let vlayer = run_vlayer_bash(&vlayer_path);
                            child_processes.push(vlayer);
                        }
                        _ => {
                            let service_name = match service {
                                InfraServices::ChainServer => "chain_server",
                                InfraServices::ChainWorker => "chain_worker",
                                _ => unreachable!(),
                            };
                            let docker_path = format!("{vlayer_path}/docker/{service_name}");
                            println!("Running docker run --build in: {docker_path}");
                            let child = std::process::Command::new("docker")
                                .args(["run", "--build", "-f", "Dockerfile.nightly", "."])
                                .current_dir(&docker_path)
                                .spawn()
                                .expect("Failed to start docker run");
                            child_processes.push(child);
                        }
                    }
                }
                InfraCommands::Stop { command } => {
                    let service = &command;
                    match service {
                        InfraServices::WebProof => {
                            let docker_path = format!("{vlayer_path}/docker/web-proof");
                            println!("Stopping docker compose in: {docker_path}");
                            std::process::Command::new("docker-compose")
                                .args(["-f", "docker-compose-release.yaml", "down"])
                                .current_dir(&docker_path)
                                .spawn()
                                .expect("Failed to stop docker-compose");
                        }
                        _ => {
                            let service_name = match service {
                                InfraServices::ChainServer => "chain_server",
                                InfraServices::ChainWorker => "chain_worker",
                                InfraServices::Vlayer => "vlayer",
                                _ => unreachable!(),
                            };
                            let docker_path = format!("{vlayer_path}/docker/{service_name}");
                            println!("Stopping docker service in: {docker_path}");
                            std::process::Command::new("docker")
                                .args(["stop", service_name])
                                .spawn()
                                .expect("Failed to stop docker service");
                        }
                    }
                }
            }
        }
        Commands::Contracts { command } => {
            match &command {
                ContractCommands::Rebuild => {
                    let vlayer_path = config::get_vlayer_path();
                    let contracts_path = format!("{vlayer_path}/contracts");
                    commands::rebuild_contracts::rebuild_contracts(&contracts_path)
                        .expect("Failed to rebuild contracts");
                }
            }
        }
        Commands::Version => {
            println!("Vlad CLI Tool v1.0.0");
        }
    }

    // Only set up Ctrl+C handler and keep main thread alive if we have child processes
    if !child_processes.is_empty() {
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, shutting down...");
            for child in &mut child_processes {
                let _ = child.kill();
            }
            std::process::exit(0);
        })
        .expect("Error setting Ctrl+C handler");

        // Keep the main thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}

fn run_web_proof_docker(vlayer_path: &str) -> std::process::Child {
    let docker_path = format!("{vlayer_path}/docker/web-proof");
    println!("Running docker compose in: {docker_path}");
    std::process::Command::new("docker-compose")
        .args(["-f", "docker-compose-release.yaml", "up"])
        .current_dir(&docker_path)
        .spawn()
        .expect("Failed to start docker-compose")
}

fn run_vlayer_bash(vlayer_path: &str) -> std::process::Child {
    let bash_path = format!("{vlayer_path}/bash");
    println!("Running run-vlayer.sh in: {bash_path}");
    std::process::Command::new("bash")
        .arg("run-vlayer.sh")
        .current_dir(&bash_path)
        .spawn()
        .expect("Failed to start run-vlayer.sh")
}

fn run_web_app(vlayer_path: &str) -> std::process::Child {
    let test_web_app_path = format!("{vlayer_path}/packages/test-web-app");
    std::process::Command::new("bun")
        .args(["run", "dev"])
        .current_dir(&test_web_app_path)
        .spawn()
        .expect("web_app: Failed to start bun run dev")
}

fn run_browser_extension(vlayer_path: &str) -> std::process::Child {
    let browser_extension_path = format!("{vlayer_path}/packages/browser-extension");
    println!("Running bun run dev in: {browser_extension_path}");
    std::process::Command::new("bun")
        .args(["run", "dev"])
        .current_dir(&browser_extension_path)
        .spawn()
        .expect("Failed to start bun run dev")
}

fn check_and_kill_port(port: u16) -> io::Result<()> {
    // Check if port is in use
    let lsof = std::process::Command::new("lsof")
        .arg("-i")
        .arg(format!(":{port}"))
        .arg("-t")
        .output()?;

    if !lsof.stdout.is_empty() {
        print!("Port {port} is in use. Kill process? [y/N] ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == "y" {
            // Get PID and kill process
            let pid = String::from_utf8_lossy(&lsof.stdout).trim().to_string();
            Command::new("kill").arg("-9").arg(&pid).output()?;
            println!("Process on port {port} killed.");
        } else {
            println!("Port {port} is in use. Please free it before continuing.");
            std::process::exit(1);
        }
    }
    Ok(())
}

fn deploy_contracts(vlayer_path: &str) -> std::process::Child {
    let contracts_path = format!("{vlayer_path}/packages/test-web-app");
    std::process::Command::new("bun")
        .args(["run", "deploy.ts"])
        .current_dir(&contracts_path)
        .spawn()
        .expect("Failed to deploy contracts")
}
