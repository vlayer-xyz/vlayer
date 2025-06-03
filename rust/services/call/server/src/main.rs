use std::str::FromStr;

use call_server_lib::{
    Cli, Config, ProofMode,
    cli::Parser,
    config::{AuthOptions, ConfigOptionsWithVersion, JwtOptions, RpcUrl, RpcUrlOrString},
    serve,
};
use common::{extract_rpc_url_token, init_tracing};
use server_utils::set_risc0_dev_mode;
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let opts: ConfigOptionsWithVersion = cli.try_into()?;

    let secrets: Vec<String> = opts
        .config
        .rpc_urls
        .iter()
        .filter_map(|rpc_url_or_string| {
            let res = match rpc_url_or_string {
                RpcUrlOrString::RpcUrl(rpc_url) => Some(rpc_url.clone()),
                RpcUrlOrString::String(s) => RpcUrl::from_str(s).ok(),
            };
            res.and_then(|RpcUrl { url, .. }| extract_rpc_url_token(&url))
        })
        .collect();
    init_tracing(opts.config.log_format.unwrap_or_default(), secrets);

    info!("Running vlayer serve...");

    if opts.config.proof_mode == ProofMode::Fake {
        warn!("Running in fake mode. Server will not generate real proofs.");
        set_risc0_dev_mode();
    }

    if let Some(auth) = opts.config.auth.as_ref() {
        match auth {
            AuthOptions::Jwt(JwtOptions {
                public_key,
                algorithm,
            }) => info!(
                "Using JWT-based authorization with public key '{}' and algorithm '{}'.",
                public_key, algorithm
            ),
        }
    } else {
        warn!("Running without authorization.");
    }

    let config: Config = opts.try_into()?;
    debug!("Using config: {config:#?}");

    serve(config).await?;

    Ok(())
}
