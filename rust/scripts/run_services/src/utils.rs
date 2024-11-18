use std::{fmt::Display, path::PathBuf, time::Duration};

use anyhow::Context;
use tokio::{
    net::TcpStream,
    process::{Child, Command},
    task::JoinSet,
    time::{sleep, Instant},
};
use tokio_util::sync::CancellationToken;

pub fn get_external_rpc_urls(alchemy_api_key: Option<&String>) -> Vec<String> {
    if let Some(api_key) = alchemy_api_key {
        vec![
            format!("--rpc-url 11155111:https://eth-sepolia.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 1:https://eth-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 8453:https://base-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 10:https://opt-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 11155420:https://opt-sepolia.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 84532:https://base-sepolia.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 80002:https://polygon-amoy.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 421614:https://arb-sepolia.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 300:https://zksync-sepolia.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 8453:https://base-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 42161:https://arb-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 42170:https://arbnova-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 137:https://polygon-mainnet.g.alchemy.com/v2/{}", api_key),
            format!("--rpc-url 324:https://zksync-mainnet.g.alchemy.com/v2/{}", api_key),
        ]
    } else {
        vec![]
    }
}

#[derive(Default)]
pub struct ChildManager {
    join_set: JoinSet<()>,
    cancel_token: CancellationToken,
}

impl ChildManager {
    pub fn add_child(&mut self, proc_name: impl Display, mut child: Child) {
        let proc_name = proc_name.to_string();
        let cancel_token = self.cancel_token.clone();
        self.join_set.spawn(async move {
            tokio::select! {
                res = child.wait() => {
                    println!("{proc_name} exited with {res:?}");
                    cancel_token.cancel(); // Cancel other processes
                },
                _ = cancel_token.cancelled() => {
                    println!("Stopping process {proc_name}..");
                    _ = child.kill().await.map_err(|e| println!("error killing {proc_name}: {e}"));
                }
            }
        });
    }

    pub async fn stop(self) {
        self.cancel_token.cancel();
        self.join_set.join_all().await;
    }
}

pub async fn get_vlayer_home() -> anyhow::Result<PathBuf> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .await
        .with_context(|| "failed to get home dir from git")?;

    let path_str = String::from_utf8(output.stdout).with_context(|| "cannot parse git output")?;
    Ok(PathBuf::from(path_str.trim()))
}

pub async fn wait_for_port(port: u16, timeout: Duration) -> anyhow::Result<()> {
    let start_time = Instant::now();

    while start_time.elapsed() < timeout {
        if TcpStream::connect(format!("127.0.0.1:{port}"))
            .await
            .is_ok()
        {
            return Ok(());
        }
        sleep(Duration::from_secs(1)).await;
    }

    anyhow::bail!("timeout waiting for port {port} to become available")
}
