use crate::json_rpc::json_rpc;
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use crate::trace::init_tracing;
use axum::{routing::post, Router};
use tracing::info;

pub async fn serve() -> anyhow::Result<()> {
    init_tracing()?;

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, server()).await?;

    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}

fn server() -> Router {
    Router::new()
        .route("/", post(json_rpc))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::{body_to_json, body_to_string, post};
    use core::str;
    use std::env;

    use super::server;
    use axum::http::StatusCode;
    use axum_jrpc::{JsonRpcRequest, Value};
    use serde_json::json;

    fn deployed_contracts_block_no() -> u32 {
        env::var("BLOCK_NO")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2)
    }

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let app = server();
        let response = post(app, "/non_existent_http_path", &()).await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    const CALLER: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    const TO: &str = "e7f1725E7734CE288F8367e1Bb143E90bb3F0512";
    const DATA: &str = "0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002";

    #[tokio::test]
    async fn json_rpc_not_found() -> anyhow::Result<()> {
        let app = server();

        let req = JsonRpcRequest {
            method: "non_existent_json_rpc_method".to_string(),
            params: Value::Null,
            id: 1.into(),
        };
        let response = post(app, "/", &req).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            body_to_json::<Value>(response.into_body()).await?,
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32601,
                    "message": "Method `non_existent_json_rpc_method` not found",
                    "data": null
                }
            })
        );

        Ok(())
    }

    mod v_call {
        use super::*;

        #[tokio::test]
        async fn field_validation_error() -> anyhow::Result<()> {
            let app = server();

            let req = json!({
                "method": "v_call",
                "params": [{"caller": "I am not a valid address!", "to": TO, "data": DATA}, {"block_no": 0}],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "error": {
                        "code": -32602,
                        "message": "Invalid field `caller`: Odd number of digits `I am not a valid address!`",
                        "data": null
                    }
                })
            );

            Ok(())
        }

        #[tokio::test]
        async fn success() -> anyhow::Result<()> {
            let app = server();

            let req = json!({
                "method": "v_call",
                "params": [{"caller": CALLER, "to": TO, "data": DATA}, {"block_no": deployed_contracts_block_no(), "chain_id": 11155111}],
                "id": 1,
                "jsonrpc": "2.0",
            });
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": "start_contract_address: 0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512, function_selector: 0xcad0899b, evm_call_result: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]"
                    }
                })
            );

            Ok(())
        }
    }
}
