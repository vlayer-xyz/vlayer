use auto_impl::auto_impl;
use serde::Serialize;
use serde_json::{Value, json};

#[auto_impl(&)]
pub trait Method: Serialize {
    const METHOD_NAME: &'static str;

    fn request_body(&self) -> Value {
        json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": Self::METHOD_NAME,
            "params": self,
        })
    }
}
