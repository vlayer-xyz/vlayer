use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use axum_extra::extract::WithRejection;
use serde::Deserialize;
use serde_json::{json, to_string};
use thiserror::Error;

#[derive(Deserialize)]
struct JsonRpcRequest {
    method: String,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let status = StatusCode::OK;
        let body = match self {
            ServerError::JsonExtractorRejection(e) => (
                StatusCode::OK,
                Json(json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32600,
                        "message": "Invalid request: ".to_string() + &e.to_string()
                    }
                })),
            ),
        };

        (status, body).into_response()
    }
}

async fn method_not_found(
    WithRejection(Json(payload), _): WithRejection<Json<JsonRpcRequest>, ServerError>,
) -> String {
    to_string(&json!({
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32601,
            "message": format!("Method `{}` not found", payload.method),
        }
    }))
    .unwrap()
}

pub fn server() -> Router {
    Router::new().route("/", post(method_not_found))
}
