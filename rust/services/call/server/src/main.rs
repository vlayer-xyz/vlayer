use call_server_lib::{
    Cli, Config, ProofMode,
    cli::Parser,
    config::{AuthOptions, ConfigOptionsWithVersion, JwtOptions, RpcUrl},
    serve,
};
use common::{LogFormat, extract_rpc_url_token, init_tracing};
use server_utils::set_risc0_dev_mode;
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let opts: ConfigOptionsWithVersion = cli.try_into()?;

    init_tracing_with_secrets(
        opts.config.log_format.unwrap_or_default(),
        opts.config.rpc_urls.iter(),
    );

    info!("Running vlayer serve...");

    if opts.config.proof_mode == ProofMode::Fake {
        warn!("Running in fake mode. Server will not generate real proofs.");
        set_risc0_dev_mode();
    }

    log_auth_mode(opts.config.auth.as_ref());

    let config: Config = opts.try_into()?;
    debug!("Using config: {config:#?}");

    serve(config).await?;

    Ok(())
}

fn init_tracing_with_secrets<'a>(
    log_format: LogFormat,
    rpc_urls: impl IntoIterator<Item = &'a RpcUrl>,
) {
    let secrets: Vec<String> = rpc_urls
        .into_iter()
        .cloned()
        .filter_map(|RpcUrl { url, .. }| extract_rpc_url_token(&url))
        .collect();
    init_tracing(log_format, secrets);
}

fn log_auth_mode(auth: Option<&AuthOptions>) {
    if let Some(auth) = auth {
        match auth {
            AuthOptions::Jwt(JwtOptions {
                public_key,
                algorithm,
                ..
            }) => info!(
                "Using JWT-based authorization with public key '{}' and algorithm '{}'.",
                public_key, algorithm
            ),
        }
    } else {
        warn!("Running without authorization.");
    }
}
