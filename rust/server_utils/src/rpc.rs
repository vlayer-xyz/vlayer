use httpmock::{Mock, MockServer};
use reqwest::blocking::Client;
use serde_json::{json, Map, Value};

pub struct RpcServerMock {
    mock_server: MockServer,
    method: String,
}

impl RpcServerMock {
    pub fn start(method: &str) -> Self {
        let mock_server = MockServer::start();

        RpcServerMock {
            mock_server,
            method: method.into(),
        }
    }

    pub fn url(&self) -> String {
        self.mock_server.base_url()
    }

    pub fn mock<P>(&self, params: Option<P>, result: impl Into<Value>) -> Mock
    where
        P: Into<Value>,
    {
        let result = result.into();

        let request_body = self.build_request_body(params);

        let mock = self.mock_server.mock(|when, then| {
            when.method("POST")
                .path("/")
                .header("Content-Type", "application/json")
                .json_body_partial(request_body);

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

    fn build_request_body<P>(&self, params: Option<P>) -> String
    where
        P: Into<Value>,
    {
        let mut request_map = Map::new();
        request_map.insert("method".to_string(), Value::String(self.method.clone()));

        if let Some(params) = params {
            request_map.insert("params".to_string(), params.into());
        }

        serde_json::to_string(&Value::Object(request_map)).unwrap()
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

    pub fn call(&self, params: Option<Value>) -> Result<Value, reqwest::Error> {
        let id = 1; // Fixed ID for simplicity
        let request_body = if let Some(params) = params {
            json!({
                "jsonrpc": "2.0",
                "method": self.method,
                "params": params,
                "id": id
            })
        } else {
            json!({
                "jsonrpc": "2.0",
                "method": self.method,
                "id": id
            })
        };

        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()?
            .json::<Value>()?;

        if let Some(error) = response.get("error") {
            panic!("JSON-RPC Error: {}", error);
        }

        Ok(response["result"].clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_without_params() -> anyhow::Result<()> {
        let rpc_mock = RpcServerMock::start("get_data");
        let mock = rpc_mock.mock(None::<Value>, json!({"data": "some data"}));
        let rpc_client = RpcClient::new(&rpc_mock.url(), "get_data");

        let result = rpc_client.call(None)?;

        mock.assert();

        assert_eq!(result, json!({"data": "some data"}));

        Ok(())
    }

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
}
