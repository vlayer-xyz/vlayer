use crate::error::AppError;
use host::host::{Host, HostConfig};
use host::Call as HostCall;
use types::{Call, CallContext, CallResult};

pub mod types;

const LOCALHOST_RPC_URL: &str = "http://localhost:8545";

pub(crate) async fn call(params: (Call, CallContext)) -> Result<CallResult, AppError> {
    let call: HostCall = params.0.try_into()?;
    let context = params.1;

    let config = HostConfig::new(LOCALHOST_RPC_URL, context.chain_id, context.block_no);

    let _return_data = tokio::task::spawn_blocking(|| Host::try_new(config)?.run(call));

    Ok(CallResult {
        result: format!(
            "Call: caller 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f to 0x7Ad53bbA1004e46dd456316912D55dBc5D311a03 with data [0, 0]. Context: block {} chain {}.",
            context.block_no, context.chain_id
        ),
    })
}
