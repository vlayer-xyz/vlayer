use axum::{routing::post, Router};
use axum_jrpc::{JrpcResult, JsonRpcExtractor};

async fn method_not_found(req: JsonRpcExtractor) -> JrpcResult {
    let method = req.method();
    Ok(req.method_not_found(method))
}

pub fn server() -> Router {
    Router::new().route("/", post(method_not_found))
}
