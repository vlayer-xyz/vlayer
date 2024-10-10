use httpmock::{Mock, MockServer};
use reqwest::blocking::Client;
use serde::Serialize;
use serde_json::{json, Value};

pub struct RpcServerMock {
    mock_server: MockServer,
    method: String,
}

impl RpcServerMock {
    pub fn start(method: impl AsRef<str>) -> Self {
        let mock_server = MockServer::start();

        RpcServerMock {
            mock_server,
            method: method.as_ref().to_string(),
        }
    }

    pub fn url(&self) -> String {
        self.mock_server.base_url()
    }

    pub fn mock(&self, params: impl Serialize, result: impl Into<Value>) -> Mock {
        let result = result.into();

        let mock = self.mock_server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body(json!({
                    "jsonrpc": "2.0",
                    "method": self.method,
                    "params": params,
                    "id": 1
                }));

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
        });

        mock
    }
}

pub struct RpcClient {
    url: String,
    method: String,
    client: Client,
}

impl RpcClient {
    pub fn new(url: &str, method: &str) -> Self {
        Self {
            url: url.into(),
            method: method.into(),
            client: Client::new(),
        }
    }

    pub fn call(&self, params: impl Serialize) -> Result<Value, reqwest::Error> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "method": self.method,
            "params": params,
            "id": 1
        });

        let mut response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?
            .error_for_status()?
            .json::<Value>()?;

        if let Some(error) = response.get("error") {
            panic!("JSON-RPC Error: {}", error);
        }

        Ok(response["result"].take())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_with_params() -> anyhow::Result<()> {
        let rpc_mock = RpcServerMock::start("get_data");
        let mock = rpc_mock.mock(Some(json!({"key": "value"})), json!({"data": "some data"}));
        let rpc_client = RpcClient::new(&rpc_mock.url(), "get_data");

        let result = rpc_client.call(Some(json!({"key": "value"})))?;

        mock.assert();

        assert_eq!(result, json!({"data": "some data"}));

        Ok(())
    }

    #[test]
    #[should_panic(expected = "No request has been received by the mock server.")]
    fn mock_not_called_panics() {
        let rpc_mock = RpcServerMock::start("get_data");
        let mock = rpc_mock.mock(Some(json!({"key": "value"})), json!({"data": "some data"}));
        RpcClient::new(&rpc_mock.url(), "get_data");

        mock.assert();
    }

    #[test]
    fn call_without_mock_returns_error() {
        let rpc_mock = RpcServerMock::start("get_data");
        let rpc_client = RpcClient::new(&rpc_mock.url(), "unmocked_method");

        let result = rpc_client.call(Some(json!({"key": "value"})));

        assert!(
            result.unwrap_err().status().is_some(),
            "Expected an HTTP status code in the error, but it was missing."
        );
    }
}
