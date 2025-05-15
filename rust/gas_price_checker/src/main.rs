use std::{env, process};

use anyhow::{bail, Context};
use reqwest::blocking::Client;
use serde_json::{from_str, json, Value};

fn main() -> anyhow::Result<()> {
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

    let gas_price_gwei = fetch_gas_price(rpc_url)?;

    println!("Gas price: {:.4} gwei", gas_price_gwei);

    if gas_price_gwei <= threshold_gwei {
        println!("✅ Gas price is low enough → OK");
        Ok(())
    } else {
        println!("❌ Gas price is too high");
        process::exit(1);
    }
}

fn fetch_gas_price(rpc_url: &str) -> anyhow::Result<f64> {
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
        .context("Failed to make RPC request")?;

    let response_text = response.text()?;
    println!("Raw response: {}", response_text);

    let response_json: Value = from_str(&response_text).context("Failed to parse JSON response")?;

    let gas_price_hex = response_json
        .get("result")
        .and_then(|v| v.as_str())
        .context("No 'result' field in response or not a string")?;

    if gas_price_hex.is_empty() {
        bail!("Error: 'gas_price_hex' is empty");
    }

    let gas_price_wei =
        u64::from_str_radix(gas_price_hex.trim_start_matches("0x"), 16).unwrap() as f64;
    let gas_price_gwei = gas_price_wei / 1e9;

    Ok(gas_price_gwei)
}
