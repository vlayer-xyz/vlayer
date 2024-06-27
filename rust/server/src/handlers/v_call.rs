use crate::error::AppError;
use host::Call as HostCall;
use types::{Call, CallContext, CallResult};

pub mod types;

pub(crate) async fn call(params: (Call, CallContext)) -> Result<CallResult, AppError> {
    let call: HostCall = params.0.try_into()?;
    let context = params.1;

    Ok(CallResult {
        result: format!(
            "Call: caller {} to {} with data {:?}. Context: block {} chain {}.",
            call.caller, call.to, call.data, context.block_no, context.chain_id
        ),
    })
}
