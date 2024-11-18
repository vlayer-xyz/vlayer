use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use anyhow::Context;
use async_tempfile::TempDir;
use clap::Parser;
use tokio::{
    fs::File,
    process::{Child, Command},
    signal::unix::{signal, SignalKind},
};
use utils::{get_external_rpc_urls, get_vlayer_home, wait_for_port, ChildManager};

mod utils;

#[derive(Parser, Debug)]
#[clap(version = "1.0")]
struct Cli {
    #[clap(long, env, default_value = "dev")]
    proving_mode: ProvingMode,

    #[clap(long, env, default_value = "https://api.bonsai.xyz/")]
    bonsai_api_url: String,

    #[clap(long, env)]
    bonsai_api_key: Option<String>,

    #[clap(long, env)]
    alchemy_api_key: Option<String>,

    #[clap(long, env)]
    vlayer_tmp_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
enum ProvingMode {
    Dev,
    Prod,
}

impl FromStr for ProvingMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(ProvingMode::Dev),
            "prod" => Ok(ProvingMode::Prod),
            _ => Err(format!("Invalid proving mode: {s}")),
        }
    }
}

impl Display for ProvingMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProvingMode::Dev => write!(f, "dev"),
            ProvingMode::Prod => write!(f, "prod"),
        }
    }
}

impl ProvingMode {
    const fn server_arg(&self) -> &str {
        match self {
            ProvingMode::Dev => "fake",
            ProvingMode::Prod => "groth16",
        }
    }
}

async fn start_anvil(logs_dir: impl AsRef<Path>) -> anyhow::Result<Child> {
    println!("Starting Anvil...");

    let output = Command::new("which").arg("anvil").output().await?;
    anyhow::ensure!(output.status.success(), "anvil command not found");

    let log_file = File::create(logs_dir.as_ref().join("anvil.out"))
        .await
        .with_context(|| "failed to create anvil log file")?
        .try_into_std()
        .map_err(|_| anyhow::anyhow!("failed to create anvil log file"))?;

    let child = Command::new("anvil")
        .args(["-p", "8545"])
        .stdout(log_file)
        .spawn()?;

    println!("Anvil started with PID {}", child.id().unwrap_or_default());
    Ok(child)
}

async fn start_vlayer(
    vlayer_home: impl AsRef<Path>,
    logs_dir: impl AsRef<Path>,
    proof_mode: &str,
    external_urls: &[String],
) -> anyhow::Result<Child> {
    println!("Starting VLayer REST server...");

    let log_file = File::create(logs_dir.as_ref().join("vlayer_serve.out"))
        .await
        .with_context(|| "failed to create anvil log file")?
        .try_into_std()
        .map_err(|_| anyhow::anyhow!("failed to create anvil log file"))?;

    let child = Command::new("cargo")
        .arg("run")
        .args(["--bin", "vlayer"])
        .arg("serve")
        .args(["--proof", proof_mode])
        .args(["--rpc-url", "31337:http://localhost:8545"])
        .args(external_urls)
        .current_dir(vlayer_home.as_ref().join("rust"))
        .stdout(log_file)
        .env("RUST_LOG", "info")
        .env("BONSAI_API_URL", "https://api.bonsai.xyz/")
        .env("BONSAI_API_KEY", "your_bonsai_api_key")
        .spawn()?;

    println!("VLayer server started with PID {}", child.id().unwrap_or_default());

    Ok(child)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let vlayer_home = get_vlayer_home()
        .await
        .with_context(|| "failed to find VLayer home directory")?;
    let (_vlayer_tmp_dir, temp_dir_path) = if let Some(temp_dir_path) = cli.vlayer_tmp_dir {
        (None, temp_dir_path)
    } else {
        let temp_dir = TempDir::new()
            .await
            .with_context(|| "failed to create temp directory")?;
        let temp_dir_path = temp_dir.dir_path().clone();
        (Some(temp_dir), temp_dir_path)
    };
    let logs_dir = temp_dir_path.join("logs");

    tokio::fs::create_dir_all(&logs_dir)
        .await
        .with_context(|| "failed to create logs directory")?;
    println!("Saving artifacts to: {}", temp_dir_path.display());

    let server_proof_arg = cli.proving_mode.server_arg();
    let external_rpc_urls = get_external_rpc_urls(cli.alchemy_api_key.as_ref());

    println!("PROVING_MODE: {}", cli.proving_mode);
    println!("BONSAI_API_URL: {}", cli.bonsai_api_url);
    println!("SERVER_PROOF_ARG: {server_proof_arg}");
    println!("EXTERNAL_RPC_URLS: {external_rpc_urls:?}");

    println!("Starting services...");

    let mut sigterm = signal(SignalKind::terminate())?;
    let mut sigint = signal(SignalKind::interrupt())?;

    let mut child_manager = ChildManager::default();

    if let Err(e) = async {
        let anvil = start_anvil(&logs_dir).await?;
        child_manager.add_child("anvil", anvil);
        wait_for_port(8545, Duration::from_secs(15)).await?;
        println!("Anvil running on port 8545");

        let vlayer =
            start_vlayer(&vlayer_home, &logs_dir, server_proof_arg, &external_rpc_urls).await?;
        child_manager.add_child("vlayer server", vlayer);
        wait_for_port(3000, Duration::from_secs(300)).await?;
        println!("VLayer server running on port 3000");

        Ok(())
    }
    .await
    {
        child_manager.stop().await;
        return Err(e);
    }

    println!("Services have been successfully started");

    tokio::select! {
        _ = sigterm.recv() => {},
        _ = sigint.recv() => {},
    }

    println!("Stopping services...");
    child_manager.stop().await;

    Ok(())
}
