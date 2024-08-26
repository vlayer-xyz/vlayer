use axum::{routing::post, Router};
use serde_json::{from_str, json, to_string, Value};

async fn method_not_found(payload: String) -> String {
    let payload: Value = from_str(&payload).unwrap();
    to_string(&json!({
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32601,
            "message": format!("Method {} not found", payload["method"]),
            "data": null
        }
    }))
    .unwrap()
}


pub fn server() -> Router {
    Router::new().route("/", post(method_not_found))
}
