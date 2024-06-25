use crate::json_rpc::json_rpc;
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use axum::{routing::post, Router};

pub fn server() -> Router {
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

    use super::server;
    use axum::http::StatusCode;
    use axum_jrpc::{JsonRpcRequest, Value};
    use serde_json::json;

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let app = server();
        let response = post(app, "/non_existent_http_path", &()).await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    const FROM: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";

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
        use crate::handlers::v_call::CallArgsRpc;

        #[tokio::test]
        async fn v_call_field_validation_error() -> anyhow::Result<()> {
            let app = server();

            let params = CallArgsRpc::new("I am not a valid address!", TO);
            let req = JsonRpcRequest {
                method: "v_call".to_string(),
                params: serde_json::to_value(params)?,
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
                        "code": -32602,
                        "message": "Invalid field `from`: Odd number of digits `I am not a valid address!`",
                        "data": null
                    }
                })
            );

            Ok(())
        }

        #[tokio::test]
        async fn v_call_success() -> anyhow::Result<()> {
            let app = server();

            let params = CallArgsRpc::new(FROM, TO);
            let req = JsonRpcRequest {
                method: "v_call".to_string(),
                params: serde_json::to_value(params)?,
                id: 1.into(),
            };
            let response = post(app, "/", &req).await?;

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                body_to_json::<Value>(response.into_body()).await?,
                json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": {
                        "result": "Call: from 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f to 0x7Ad53bbA1004e46dd456316912D55dBc5D311a03!"
                    }
                })
            );

            Ok(())
        }
    }
}
