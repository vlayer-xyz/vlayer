use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{from_str, json, to_string, Value};

#[derive(Deserialize)]
struct Request {
    method: String
}

async fn method_not_found(Json(payload): Json<Request>) -> String {
    to_string(&json!({
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32601,
            "message": format!("Method {} not found", payload.method),
            "data": null
        }
    }))
    .unwrap()
}


pub fn server() -> Router {
    Router::new().route("/", post(method_not_found))
}
