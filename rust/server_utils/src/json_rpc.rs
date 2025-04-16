use axum::{
    body::Bytes,
    http::{header::CONTENT_TYPE, status::StatusCode},
    response::IntoResponse,
};
use derive_new::new;
use jsonrpsee::{
    ConnectionId, Extensions, MethodCallback, MethodResponse, RpcModule,
    types::{
        Id, Request,
        error::{self as jrpcerror, ErrorObjectOwned},
    },
};
use mime::APPLICATION_JSON;
use tracing::{error, info};

#[derive(new, Clone)]
pub struct Router<T: Send + Sync + Clone + 'static>(RpcModule<T>);

impl<T> Router<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub async fn handle_request(mut self, body: Bytes) -> impl IntoResponse {
        let extensions = self.0.extensions().clone();
        self.handle(body, extensions).await
    }

    pub async fn handle_request_with_params<Params>(
        mut self,
        body: Bytes,
        params: Params,
    ) -> impl IntoResponse
    where
        Params: Clone + Send + Sync + 'static,
    {
        let mut extensions = self.0.extensions().clone();
        extensions.insert(params);
        self.handle(body, extensions).await
    }

    async fn handle(self, body: Bytes, extensions: Extensions) -> impl IntoResponse {
        let response = match serde_json::from_slice::<Request>(&body) {
            Ok(request) => self.handle_inner(request, extensions).await,
            Err(err) => MethodResponse::error(Id::Null, Error::InvalidRequest(err)),
        };
        log_response(&response);
        (
            StatusCode::OK,
            [(CONTENT_TYPE, APPLICATION_JSON.to_string())],
            response.to_result(),
        )
    }

    async fn handle_inner(self, request: Request<'_>, extensions: Extensions) -> MethodResponse {
        let id = request.id().into_owned();
        let params = request.params().into_owned();
        let conn_id = ConnectionId(0);
        match self.0.method(request.method_name()) {
            Some(method) => match method {
                MethodCallback::Async(cb) => cb(id, params, conn_id, usize::MAX, extensions).await,
                _ => todo!("implement other method types in handler"),
            },
            None => MethodResponse::error(id, Error::MethodNotFound(request.method_name().into())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Method `{0}` not found")]
    MethodNotFound(String),
    #[error("{0}")]
    InvalidRequest(#[from] serde_json::error::Error),
}

impl From<Error> for ErrorObjectOwned {
    fn from(error: Error) -> Self {
        match error {
            Error::MethodNotFound(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::METHOD_NOT_FOUND_CODE,
                error.to_string(),
                None,
            ),
            Error::InvalidRequest(..) => ErrorObjectOwned::owned::<()>(
                jrpcerror::INVALID_REQUEST_CODE,
                error.to_string(),
                None,
            ),
        }
    }
}

#[allow(clippy::unwrap_used)]
fn log_response(response: &MethodResponse) {
    if response.is_success() {
        info!(result = response.as_result(), "JsonRpc request success")
    } else {
        error!(
            code = response.as_error_code().unwrap(),
            result = response.as_result(),
            "JsonRpc request failed"
        );
    }
}
