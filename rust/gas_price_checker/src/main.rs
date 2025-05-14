use std::{env, process};

use alloy_primitives::U256;
use anyhow::{anyhow, bail, Context};
use reqwest::Client;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <RPC_URL> [THRESHOLD_GWEI]", args[0]);
        process::exit(1);
    }

    let rpc_url = &args[1];
    let threshold_gwei = args
        .get(2)
        .map(|s| s.parse::<f64>().unwrap_or(10.0))
        .unwrap_or(10.0);

    let gas_price_gwei = fetch_gas_price(rpc_url).await?;

    println!("Gas price: {:.4} gwei", gas_price_gwei);

    if gas_price_gwei <= threshold_gwei {
        println!("✅ Gas price is low enough → OK");
        Ok(())
    } else {
        println!("❌ Gas price is too high → SKIP");
        process::exit(1);
    }
}

async fn fetch_gas_price(rpc_url: &str) -> anyhow::Result<f64> {
    let client = Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "method": "eth_gasPrice",
        "params": [],
        "id": 1
    });

    let response = client
        .post(rpc_url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .context("Failed to make RPC request")?;

    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);

    let response_json: Value =
        serde_json::from_str(&response_text).context("Failed to parse JSON response")?;

    let gas_price_hex = response_json
        .get("result")
        .and_then(|v| v.as_str())
        .context("No 'result' field in response or not a string")?;

    if gas_price_hex.is_empty() || gas_price_hex == "null" {
        bail!("Error: Failed to fetch gas price");
    }

    let gas_price_wei = U256::from_str_radix(gas_price_hex.trim_start_matches("0x"), 16)
        .map_err(|e| anyhow!("Failed to parse hex gas price: {}", e))?;

    let as_str = gas_price_wei.to_string();
    let wei_f64: f64 = as_str
        .parse()
        .context("Failed to parse gas price string into f64")?;
    let gas_price_gwei = wei_f64 / 1e9;

    dbg!(gas_price_gwei);

    Ok(gas_price_gwei)
}
