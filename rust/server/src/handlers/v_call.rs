use crate::error::AppError;
use host::{
    host::{Host, HostConfig},
    Call as HostCall,
};
use types::{Call, CallContext, CallResult};

pub mod types;

const LOCALHOST_RPC_URL: &str = "http://localhost:8545";

pub(crate) async fn call(params: (Call, CallContext)) -> Result<CallResult, AppError> {
    let call: HostCall = params.0.try_into()?;
    let context = params.1;

    let _return_data = Host::try_new(HostConfig::new(
        LOCALHOST_RPC_URL,
        context.chain_id,
        context.block_no,
    ))?
    .run(call.clone())?;

    Ok(CallResult {
        result: format!(
            "Call: caller {} to {} with data {:?}. Context: block {} chain {}.",
            call.caller, call.to, call.data, context.block_no, context.chain_id
        ),
    })
}
