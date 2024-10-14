use std::mem::take;

use httpmock::{Mock, MockServer};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error;

pub struct RpcServerMock {
    mock_server: MockServer,
    method: String,
}

impl RpcServerMock {
    pub async fn start(method: impl AsRef<str>) -> Self {
        let mock_server = MockServer::start_async().await;

        RpcServerMock {
            mock_server,
            method: method.as_ref().to_string(),
        }
    }

    pub fn url(&self) -> String {
        self.mock_server.base_url()
    }

    pub async fn mock(
        &self,
        is_partial: bool,
        params: impl Serialize,
        result: impl Serialize,
    ) -> Mock {
        self.mock_server
            .mock_async(move |mut when, then| {
                when = when
                    .method("POST")
                    .path("/")
                    .header("Content-Type", "application/json");

                if is_partial {
                    let partial_body = json!({
                        "jsonrpc": "2.0",
                        "method": self.method,
                        "params": params,
                        "id": 1
                    });
                    when.json_body_partial(serde_json::to_string(&partial_body).unwrap());
                } else {
                    when.json_body(json!({
                        "jsonrpc": "2.0",
                        "method": self.method,
                        "params": params,
                        "id": 1
                    }));
                }

                then.status(200)
                    .header("Content-Type", "application/json")
                    .body(
                        serde_json::to_string(&json!({
                            "jsonrpc": "2.0",
                            "result": result,
                            "id": 1
                        }))
                        .unwrap(),
                    );
            })
            .await
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

fn parse_json_rpc_response(response_body: Value) -> Result<Value, RpcError> {
    let mut response = response_body;
    let error = extract_field(&mut response, "error");
    let result = extract_field(&mut response, "result");

    match (error, result) {
        (Some(_), Some(_)) => Err(RpcError::InvalidResponse(response)),
        (Some(error), None) => Err(RpcError::JsonRpc(error)),
        (None, Some(result)) => Ok(result),
        (None, None) => Err(RpcError::MissingResult),
    }
}

fn extract_field(response: &mut Value, field: &str) -> Option<Value> {
    response
        .get_mut(field)
        .and_then(|v| (!v.is_null()).then(|| std::mem::take(v)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_with_params() -> anyhow::Result<()> {
        let rpc_mock = RpcServerMock::start("get_data").await;
        let mock = rpc_mock
            .mock(false, json!({"key": "value"}), json!({"data": "some data"}))
            .await;
        let rpc_client = RpcClient::new(&rpc_mock.url(), "get_data");

        let result = rpc_client.call(json!({"key": "value"})).await?;

        mock.assert();

        assert_eq!(result, json!({"data": "some data"}));

        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "No request has been received by the mock server.")]
    async fn mock_not_called_panics() {
        let rpc_mock = RpcServerMock::start("get_data").await;
        let mock = rpc_mock
            .mock(false, json!({"key": "value"}), json!({"data": "some data"}))
            .await;

        mock.assert();
    }

    #[tokio::test]
    async fn call_without_mock_returns_error() {
        let rpc_mock = RpcServerMock::start("mocked_method").await;
        let rpc_client = RpcClient::new(&rpc_mock.url(), "unmocked_method");

        let result = rpc_client.call(json!({"key": "value"})).await;

        assert!(matches!(result, Err(RpcError::Http(_))));
    }

    #[tokio::test]
    async fn mock_partial_matches_full_body() -> anyhow::Result<()> {
        let rpc_mock = RpcServerMock::start("get_data").await;
        let mock = rpc_mock
            .mock(
                true,
                json!({
                    "key": "value"
                }),
                json!({"data": "some data"}),
            )
            .await;
        let rpc_client = RpcClient::new(&rpc_mock.url(), "get_data");

        let result = rpc_client.call(json!({"key": "value"})).await?;

        mock.assert();

        assert_eq!(result, json!({"data": "some data"}));

        Ok(())
    }

    #[tokio::test]
    async fn mock_non_partial_doesnt_match_full_body() -> anyhow::Result<()> {
        let rpc_mock = RpcServerMock::start("get_data").await;
        rpc_mock
            .mock(false, json!({}), json!({"data": "some data"}))
            .await;
        let rpc_client = RpcClient::new(&rpc_mock.url(), "get_data");

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