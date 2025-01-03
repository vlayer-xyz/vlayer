use call_server_lib::serve;
use tracing::{info, warn};

use crate::{
    commands::{args::ServeArgs, version::version},
    errors::CLIError,
};

pub(crate) async fn run_serve(serve_args: ServeArgs) -> Result<(), CLIError> {
    let api_version = version();
    let server_config = serve_args.into_server_config(api_version);

    info!("Running vlayer serve...");
    if server_config.fake_proofs() {
        warn!("Running in fake mode. Server will not generate real proofs.");
    }
    serve(server_config).await?;
    Ok(())
}
