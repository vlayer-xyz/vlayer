use call_guest_wrapper::GUEST_ELF as CALL_GUEST_ELF;
use call_server::{serve, Config};
use chain_guest_wrapper::GUEST_ELF as CHAIN_GUEST_ELF;
use mock_chain_server::{ChainProofServerMock, EMPTY_PROOF_RESPONSE};
use tracing::{info, warn};

use crate::{
    commands::{args::ServeArgs, version::version},
    errors::CLIError,
};

async fn start_chain_proof_server() -> String {
    warn!("Chain proof sever URL not provided. Running with mock server");
    let mut chain_proof_server = ChainProofServerMock::start().await;
    chain_proof_server
        .mock_chain_proof()
        .with_result(EMPTY_PROOF_RESPONSE.clone())
        .add()
        .await;
    chain_proof_server.url()
}

pub async fn args_to_server_config(args: ServeArgs, api_version: String) -> Config {
    let proof_mode = args.proof.unwrap_or_default().map();
    let chain_proof_url = match args.chain_proof_url {
        Some(url) => url,
        None => start_chain_proof_server().await,
    };
    call_server::ConfigBuilder::new(
        chain_proof_url,
        CALL_GUEST_ELF.clone(),
        CHAIN_GUEST_ELF.clone(),
        api_version,
    )
    .with_rpc_mappings(args.rpc_url)
    .with_proof_mode(proof_mode)
    .with_host(args.host)
    .with_port(args.port)
    .build()
}

pub(crate) async fn run_serve(serve_args: ServeArgs) -> Result<(), CLIError> {
    let api_version = version();
    let server_config = args_to_server_config(serve_args, api_version).await;

    info!("Running vlayer serve...");
    if server_config.fake_proofs() {
        warn!("Running in fake mode. Server will not generate real proofs.");
    }
    serve(server_config).await?;
    Ok(())
}
