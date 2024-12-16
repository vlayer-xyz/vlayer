use axum::{
    body::Bytes,
    http::{header::CONTENT_TYPE, status::StatusCode},
    response::IntoResponse,
};
use derive_new::new;
use jsonrpsee::{
    types::{
        error::{self as jrpcerror, ErrorObjectOwned},
        Id, Request,
    },
    ConnectionId, MethodCallback, MethodResponse, RpcModule,
};
use mime::APPLICATION_JSON;

#[derive(new, Clone)]
pub struct Router<T: Send + Sync + Clone + 'static>(RpcModule<T>);

impl<T> Router<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub async fn handle_request(self, body: Bytes) -> impl IntoResponse {
        let response = match serde_json::from_slice::<Request>(&body) {
            Ok(request) => self.handle_request_inner(request).await,
            Err(err) => {
                let err = Error::InvalidJson(err);
                MethodResponse::error(Id::Null, err)
            }
        };
        (
            StatusCode::OK,
            [(CONTENT_TYPE, APPLICATION_JSON.to_string())],
            response.to_result(),
        )
            .into_response()
    }

    async fn handle_request_inner(mut self, request: Request<'_>) -> MethodResponse {
        let id = request.id().into_owned();
        let params = request.params().into_owned();
        let exts = self.0.extensions().clone();
        let conn_id = ConnectionId(0);
        if let Some(method) = self.0.method(request.method_name()) {
            match method {
                MethodCallback::Async(cb) => cb(id, params, conn_id, usize::MAX, exts).await,
                _ => todo!("implement other method types in handler"),
            }
        } else {
            let err = Error::MethodNotFound(request.method_name().to_string());
            MethodResponse::error(id, err)
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Method `{0}` not found")]
    MethodNotFound(String),
    #[error("{0}")]
    InvalidJson(#[from] serde_json::error::Error),
}

impl From<Error> for ErrorObjectOwned {
    fn from(error: Error) -> Self {
        match error {
            Error::MethodNotFound(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::METHOD_NOT_FOUND_CODE,
                error.to_string(),
                None,
            ),
            Error::InvalidJson(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_REQUEST_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}
