use std::mem::take;

pub use common::Method;
use reqwest::Client as RawClient;
use serde_json::Value;
use tracing::info;

pub struct Client {
    url: String,
    client: RawClient,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON-RPC error: {0}")]
    JsonRpc(Value),
    #[error("Missing 'result' field in the response")]
    MissingResult,
    #[error("Invalid response: {0}")]
    InvalidResponse(Value),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct RequestBuilder(reqwest::RequestBuilder);

impl RequestBuilder {
    #[must_use]
    pub fn with_header(mut self, name: &str, value: &str) -> Self {
        self.0 = self.0.header(name, value);
        self
    }

    pub async fn send(self) -> Result<Value> {
        let response = self.0.send().await?.error_for_status()?;
        let response_body = response.json::<Value>().await?;
        let response_json = parse_json_rpc_response(response_body)?;
        info!("  <= {response_json}");
        Ok(response_json)
    }
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.into(),
            client: RawClient::new(),
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn request<M>(&self, method: M) -> RequestBuilder
    where
        M: Method,
    {
        let request_body = method.request_body();
        info!("{} => {}", M::METHOD_NAME, request_body);
        let builder = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body);
        RequestBuilder(builder)
    }

    pub async fn call(&self, method: impl Method) -> Result<Value> {
        self.request(method).send().await
    }
}

fn parse_json_rpc_response(response_body: Value) -> Result<Value> {
    let mut response = response_body;
    let error = response.get_mut("error").map(take);
    let result = response.get_mut("result").map(take);

    match (error, result) {
        (Some(_), Some(_)) => Err(Error::InvalidResponse(response)),
        (Some(error), None) => Err(Error::JsonRpc(error)),
        (None, Some(result)) => Ok(result),
        (None, None) => Err(Error::MissingResult),
    }
}

pub mod mock {
    use derive_new::new;
    use mockito::{Matcher, Mock, ServerGuard};
    use serde::Serialize;
    use serde_json::json;

    #[derive(new)]
    pub struct Server {
        server: ServerGuard,
        mocks: Vec<Mock>,
    }

    impl Server {
        pub async fn start() -> Self {
            Self::new(mockito::Server::new_async().await, Vec::new())
        }

        #[must_use]
        pub fn mock_method<'a>(&'a mut self, method_name: &'a str) -> MockBuilder<'a> {
            let mock = self
                .server
                .mock("POST", "/")
                .match_header("Content-Type", "application/json");
            MockBuilder::new(self, method_name, mock)
        }

        pub fn url(&self) -> String {
            self.server.url()
        }

        pub fn assert(&self) {
            for mock in &self.mocks {
                mock.assert();
            }
        }
    }

    #[derive(new)]
    pub struct MockBuilder<'a> {
        server: &'a mut Server,
        method_name: &'a str,
        mock: Mock,
    }

    impl<'a> MockBuilder<'a> {
        #[must_use]
        pub fn with_params(mut self, params: impl Serialize, is_partial_match: bool) -> Self {
            let request_body = json!({
                "id": 1,
                "jsonrpc": "2.0",
                "method": self.method_name,
                "params": params,
            });
            self.mock = self.mock.match_body(if is_partial_match {
                Matcher::PartialJson(request_body)
            } else {
                Matcher::Json(request_body)
            });
            self
        }

        #[must_use]
        pub fn with_result(mut self, result: impl Serialize) -> Self {
            let response_body = json!({
                "jsonrpc": "2.0",
                "result": result,
                "id": 1
            });
            self.mock = self.mock.with_body(response_body.to_string());
            self
        }

        #[must_use]
        pub fn with_expected_calls(mut self, expected_calls: usize) -> Self {
            self.mock = self.mock.expect(expected_calls);
            self
        }

        pub async fn add(self) {
            let mock = self.mock.create_async().await;
            self.server.mocks.push(mock);
        }
    }
}

#[cfg(test)]
mod tests {
    use derive_new::new;
    use mock::Server;
    use serde::Serialize;
    use serde_json::json;

    use super::*;

    #[derive(new, Serialize)]
    struct GetData {
        key: String,
    }

    impl Method for GetData {
        const METHOD_NAME: &str = "get_data";
    }

    async fn start_server(
        is_partial: bool,
        params: impl Serialize,
        response: impl Serialize,
    ) -> Server {
        let mut server = Server::start().await;
        server
            .mock_method(GetData::METHOD_NAME)
            .with_params(params, is_partial)
            .with_result(response)
            .add()
            .await;
        server
    }

    #[tokio::test]
    async fn mock_with_params() -> anyhow::Result<()> {
        let params = GetData::new("value".into());
        let expected_response = json!({"data": "some_data"});
        let mock = start_server(false, &params, &expected_response).await;
        let rpc_client = Client::new(&mock.url());

        let response = rpc_client.call(params).await?;

        mock.assert();

        assert_eq!(response, expected_response);

        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "Expected 1 request(s) to:")]
    async fn mock_not_called_panics() {
        start_server(false, json!({}), json!({})).await.assert();
    }

    #[tokio::test]
    async fn call_without_mock_returns_error() {
        let params = GetData::new("value".into());
        let mock = start_server(false, json!({}), json!({})).await;
        let rpc_client = Client::new(&mock.url());

        let result = rpc_client.call(params).await;

        assert!(matches!(result, Err(Error::Http(_))));
    }

    #[tokio::test]
    async fn mock_partial_matches_full_body() -> anyhow::Result<()> {
        let params = GetData::new("value".into());
        let mock = start_server(true, json!({}), json!({"data": "some_data"})).await;
        let rpc_client = Client::new(&mock.url());

        let result = rpc_client.call(params).await?;

        mock.assert();

        assert_eq!(result, json!({"data": "some_data"}));

        Ok(())
    }

    #[tokio::test]
    async fn mock_non_partial_doesnt_match_full_body() -> anyhow::Result<()> {
        let params = GetData::new("value".into());
        let mock = start_server(false, json!({}), json!({"data": "some_data"})).await;
        let rpc_client = Client::new(&mock.url());

        let result = rpc_client.call(params).await;

        assert!(result.is_err());

        Ok(())
    }
}
