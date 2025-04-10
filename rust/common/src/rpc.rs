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

// RPC URLs typically contain a secret token in path.
// Extract it for the purpose of redacting secret values in logs.
pub fn extract_rpc_url_token(rpc_url: &str) -> Option<String> {
    if let Ok(parsed_url) = url::Url::parse(rpc_url) {
        if let Some(path) = parsed_url.path_segments() {
            let token = path.collect::<Vec<_>>().join("/");
            if !token.is_empty() {
                return Some(token);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_rpc_url_token_with_http() {
        let url = "http://example.com/secret-token";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, Some("secret-token".to_string()));
    }

    #[test]
    fn test_extract_rpc_url_token_with_https() {
        let url = "https://example.com/another-token";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, Some("another-token".to_string()));
    }

    #[test]
    fn test_extract_rpc_url_token_no_token() {
        let url = "https://example.com/";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_rpc_url_token_no_path() {
        let url = "https://example.com";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_rpc_url_token_empty_url() {
        let url = "";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, None);
    }

    #[test]
    fn test_extract_rpc_url_token_with_port() {
        let url = "https://example.com:8080/port-token";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, Some("port-token".to_string()));
    }

    #[test]
    fn test_extract_rpc_url_token_with_query_params() {
        let url = "https://example.com/token?query=param";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, Some("token".to_string()));
    }

    #[test]
    fn test_extract_rpc_url_token_with_multiple_path_segments() {
        let url = "https://example.com/path1/path2";
        let result = extract_rpc_url_token(url);
        assert_eq!(result, Some("path1/path2".to_string()));
    }
}
