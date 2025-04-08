use auto_impl::auto_impl;
use serde::Serialize;
use serde_json::{json, Value};

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

// RPC URLs typically contain a secret token in path.
// Extract it for the purpose of redacting secret values in logs.
pub fn extract_rpc_url_token(rpc_url: &String) -> Option<String> {
    let stripped_url = rpc_url
        .trim_start_matches("http://")
        .trim_start_matches("https://");

    // Extract the part after the first '/' if it exists
    if let Some(pos) = stripped_url.find('/') {
        let token = &stripped_url[pos + 1..];
        if !token.is_empty() {
            return Some(token.to_string());
        }
    }
    None
}
