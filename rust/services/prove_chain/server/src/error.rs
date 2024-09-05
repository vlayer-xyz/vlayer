use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {}

impl From<Error> for JsonRpcError {
    fn from(_: Error) -> Self {
        JsonRpcError::new(
            JsonRpcErrorReason::InternalError,
            "".to_string(),
            Value::Null,
        )
    }
}
