use std::mem::take;

use derive_new::new;
use mockito::{Matcher, Mock, ServerGuard};
use reqwest::Client;
use serde::Serialize;
use serde_json::{json, Value};
use thiserror::Error;
use tracing::info;

#[derive(new)]
pub struct RpcServerMock {
    server: ServerGuard,
    mocks: Vec<Mock>,
}

impl RpcServerMock {
    pub async fn start() -> Self {
        Self::new(mockito::Server::new_async().await, Vec::new())
    }

    pub async fn add_mock(
        &mut self,
        method: impl AsRef<str>,
        params: impl Serialize,
        result: impl Serialize,
        is_partial: bool,
        expected_calls: usize,
    ) {
        let request_body = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": method.as_ref(),
            "params": params,
        });

        let response_body = json!({
            "jsonrpc": "2.0",
            "result": result,
            "id": 1
        });

        let mut mock = self
            .server
            .mock("POST", "/")
            .match_header("Content-Type", "application/json");

        mock = mock.match_body(if is_partial {
            Matcher::PartialJson(request_body.clone())
        } else {
            Matcher::Json(request_body)
        });

        mock = mock
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(response_body.to_string())
            .expect(expected_calls)
            .create_async()
            .await;

        self.mocks.push(mock);
    }

    pub fn url(&self) -> String {
        self.server.url()
    }

    pub fn assert(&self) {
        if !self.mocks.iter().all(Mock::matched) {
            panic!("wrong number of requests. expected 1");
        }
    }
}

pub struct RpcClient {
    url: String,
    client: Client,
}

pub trait RpcMethod: Serialize {
    const METHOD_NAME: &str;

    fn request_body(&self) -> Value {
        json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": Self::METHOD_NAME,
            "params": self,
        })
    }
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
    pub fn new(url: &str) -> Self {
        Self {
            url: url.into(),
            client: Client::new(),
        }
    }

    pub async fn call<Method>(&self, method: Method) -> Result<Value, RpcError>
    where
        Method: RpcMethod,
    {
        let request_body = method.request_body();
        info!("{} => {}", Method::METHOD_NAME, request_body);

        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;

        let response_body = response.json::<Value>().await?;

        let response_json = parse_json_rpc_response(response_body)?;
        info!("  <= {response_json}");
        Ok(response_json)
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
    use derive_new::new;

    use super::*;

    #[derive(new, Serialize)]
    struct GetData {
        key: String,
    }

    impl RpcMethod for GetData {
        const METHOD_NAME: &str = "get_data";
    }

    async fn server_start(
        is_partial: bool,
        params: impl Serialize,
        response: impl Serialize,
    ) -> RpcServerMock {
        let mut server = RpcServerMock::start().await;
        server
            .add_mock(GetData::METHOD_NAME, params, response, is_partial, 1)
            .await;
        server
    }

    #[tokio::test]
    async fn mock_with_params() -> anyhow::Result<()> {
        let params = GetData::new("value".into());
        let expected_response = json!({"data": "some_data"});
        let mock = server_start(false, &params, &expected_response).await;
        let rpc_client = RpcClient::new(&mock.url());

        let response = rpc_client.call(params).await?;

        mock.assert();

        assert_eq!(response, expected_response);

        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "wrong number of requests. expected 1")]
    async fn mock_not_called_panics() {
        server_start(false, json!({}), json!({})).await.assert();
    }

    #[tokio::test]
    async fn call_without_mock_returns_error() {
        let params = GetData::new("value".into());
        let mock = server_start(false, json!({}), json!({})).await;
        let rpc_client = RpcClient::new(&mock.url());

        let result = rpc_client.call(params).await;

        assert!(matches!(result, Err(RpcError::Http(_))));
    }

    #[tokio::test]
    async fn mock_partial_matches_full_body() -> anyhow::Result<()> {
        let params = GetData::new("value".into());
        let mock = server_start(true, json!({}), json!({"data": "some_data"})).await;
        let rpc_client = RpcClient::new(&mock.url());

        let result = rpc_client.call(params).await?;

        mock.assert();

        assert_eq!(result, json!({"data": "some_data"}));

        Ok(())
    }

    #[tokio::test]
    async fn mock_non_partial_doesnt_match_full_body() -> anyhow::Result<()> {
        let params = GetData::new("value".into());
        let mock = server_start(false, json!({}), json!({"data": "some_data"})).await;
        let rpc_client = RpcClient::new(&mock.url());

        let result = rpc_client.call(params).await;

        assert!(result.is_err());

        Ok(())
    }
}
