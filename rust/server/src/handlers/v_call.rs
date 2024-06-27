use super::v_call_types::{Call, CallContext, CallResult};
use crate::error::AppError;
use host::Call as HostCall;

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
