use std::mem::take;

use mockito::{Matcher, Mock, ServerGuard};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error;

pub struct RpcServerMock {
    server: ServerGuard,
    mock: Mock,
}

impl RpcServerMock {
    pub async fn start(
        method: impl AsRef<str>,
        is_partial: bool,
        params: impl Serialize,
        result: impl Serialize,
    ) -> Self {
        let mut server = mockito::Server::new_async().await;

        let request_body = json!({
            "jsonrpc": "2.0",
            "method": method.as_ref(),
            "params": params,
            "id": 1
        });

        let response_body = json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": 1
        });

        let mock = if is_partial {
            server
                .mock("POST", "/")
                .match_header("Content-Type", "application/json")
                .match_body(Matcher::PartialJson(request_body.clone()))
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(response_body.to_string())
                .create_async()
                .await
        } else {
            server
                .mock("POST", "/")
                .match_header("Content-Type", "application/json")
                .match_body(Matcher::Json(request_body))
                .with_status(200)
                .with_header("Content-Type", "application/json")
                .with_body(response_body.to_string())
                .create_async()
                .await
        };

        RpcServerMock { server, mock }
    }

    pub fn url(&self) -> String {
        self.server.url()
    }

    pub fn assert(&self) {
        self.mock.assert();
    }
}

pub struct RpcClient {
    url: String,
    method: String,
    client: Client,
}

#[derive(Debug, Error)]
pub enum RpcError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON-RPC error: {0}")]
    JsonRpc(Value),
    #[error("Missing 'result' field in the response")]
    MissingResult,
    #[error("Invalid response: {0}")]
    InvalidResponse(Value),
}

impl RpcClient {
    pub fn new(url: &str, method: &str) -> Self {
        Self {
            url: url.into(),
            method: method.into(),
            client: Client::new(),
        }
    }

    pub async fn call(&self, params: impl Serialize) -> Result<Value, RpcError> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": self.method,
            "params": params,
            "id": 1
        });

        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        let response_body = response.json::<Value>().await?;

        parse_json_rpc_response(response_body)
    }
}

fn parse_json_rpc_response(response_body: Value) -> Result<Value, RpcError> {
    let mut response = response_body;
    let error = response.get_mut("error").map(take);
    let result = response.get_mut("result").map(take);

    match (error, result) {
        (Some(_), Some(_)) => Err(RpcError::InvalidResponse(response)),
        (Some(error), None) => Err(RpcError::JsonRpc(error)),
        (None, Some(result)) => Ok(result),
        (None, None) => Err(RpcError::MissingResult),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const METHOD: &str = "get_data";

    #[tokio::test]
    async fn mock_with_params() -> anyhow::Result<()> {
        let params = json!({"key": "value"});
        let expected_response = json!({"data": "some data"});
        let mock = RpcServerMock::start(METHOD, false, &params, &expected_response).await;
        let rpc_client = RpcClient::new(&mock.url(), METHOD);

        let response = rpc_client.call(params).await?;

        mock.assert();

        assert_eq!(response, expected_response);

        Ok(())
    }

    #[tokio::test]
    #[should_panic]
    async fn mock_not_called_panics() {
        let mock = RpcServerMock::start(METHOD, false, json!({}), json!({})).await;

        mock.assert();
    }

    #[tokio::test]
    async fn call_without_mock_returns_error() {
        let mock = RpcServerMock::start(METHOD, false, json!({}), json!({})).await;
        let rpc_client = RpcClient::new(&mock.url(), METHOD);

        let result = rpc_client.call(json!({"key": "value"})).await;

        assert!(matches!(result, Err(RpcError::Http(_))));
    }

    #[tokio::test]
    async fn mock_partial_matches_full_body() -> anyhow::Result<()> {
        let mock =
            RpcServerMock::start(METHOD, true, json!({}), json!({"data": "some data"})).await;
        let rpc_client = RpcClient::new(&mock.url(), METHOD);

        let result = rpc_client.call(json!({"key": "value"})).await?;

        mock.assert();

        assert_eq!(result, json!({"data": "some data"}));

        Ok(())
    }

    #[tokio::test]
    async fn mock_non_partial_doesnt_match_full_body() -> anyhow::Result<()> {
        let mock =
            RpcServerMock::start(METHOD, false, json!({}), json!({"data": "some data"})).await;
        let rpc_client = RpcClient::new(&mock.url(), METHOD);

        let result = rpc_client
            .call(json!({
                    "key": "value",
                }
            ))
            .await;

        assert!(result.is_err());

        Ok(())
    }
}
