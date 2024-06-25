use crate::json_rpc::json_rpc;
use crate::layers::request_id::RequestIdLayer;
use crate::layers::trace::init_trace_layer;
use axum::{routing::post, Router};

pub fn app() -> Router {
    Router::new()
        .route("/", post(json_rpc))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

#[cfg(test)]
mod tests {
    use core::str;

    use crate::handlers::v_call::Call;

    use super::app;
    use axum::{
        body::Body,
        http::{header::CONTENT_TYPE, Request, Response, StatusCode},
        Router,
    };
    use axum_jrpc::{JsonRpcRequest, Value};
    use http_body_util::BodyExt;
    use mime::APPLICATION_JSON;
    use serde::{de::DeserializeOwned, Serialize};
    use serde_json::{json, to_string};
    use tower::ServiceExt;

    async fn post<T>(app: Router, url: &str, body: &T) -> anyhow::Result<Response<Body>>
    where
        T: Serialize,
    {
        let request = Request::post(url)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .body(Body::from(to_string(body)?))?;
        Ok(app.oneshot(request).await?)
    }

    async fn body_to_string(body: Body) -> anyhow::Result<String> {
        let body_bytes = body.collect().await?.to_bytes();
        Ok(String::from_utf8(body_bytes.to_vec())?)
    }

    async fn body_to_json<T: DeserializeOwned>(body: Body) -> anyhow::Result<T> {
        let body_bytes = body.collect().await?.to_bytes();
        let deserialized = serde_json::from_slice(&body_bytes)?;
        Ok(deserialized)
    }

    #[tokio::test]
    async fn http_not_found() -> anyhow::Result<()> {
        let app = app();
        let response = post(app, "/non_existent_http_path", &()).await?;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        assert!(body_to_string(response.into_body()).await?.is_empty());

        Ok(())
    }

    const CALLER: &str = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f";
    const TO: &str = "0x7Ad53bbA1004e46dd456316912D55dBc5D311a03";

    #[tokio::test]
    async fn json_rpc_not_found() -> anyhow::Result<()> {
        let app = app();

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

    #[tokio::test]
    async fn v_call_field_validation_error() -> anyhow::Result<()> {
        let app = app();

        let params = Call::new("I am not a valid address!", TO);
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
                    "message": "Invalid field `caller`: Odd number of digits `I am not a valid address!`",
                    "data": null
                }
            })
        );

        Ok(())
    }

    #[tokio::test]
    async fn v_call_success() -> anyhow::Result<()> {
        let app = app();

        let params = Call::new(CALLER, TO);
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
                    "result": "Call: caller 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f to 0x7Ad53bbA1004e46dd456316912D55dBc5D311a03!"
                }
            })
        );

        Ok(())
    }
}
