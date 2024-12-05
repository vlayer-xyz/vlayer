use serde::Serialize;
use serde_json::{json, Value};

pub trait Method: Serialize {
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
