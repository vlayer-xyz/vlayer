use std::{collections::HashMap, env};

use alloy_primitives::ChainId;
use derive_new::new;
use jsonrpsee::http_client::HttpClientBuilder;
use thiserror::Error;

use crate::{
    IClient,
    client::{FactoryError, IFactory, http},
};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum Error {
    #[error("HttpClientBuilder error: {0}")]
    HttpClientBuilder(String),
    #[error("No RPC URL for chain {0}")]
    NoRpcUrl(ChainId),
}

#[derive(Debug, Clone, new, Default)]
pub struct Factory {
    rpc_urls: HashMap<ChainId, String>,
}

impl Factory {
    fn get_rollup_endpoint_override(chain_id: ChainId) -> Option<String> {
        let env_var = match chain_id {
            10 => "OPTIMISM_ROLLUP_ENDPOINT",
            11_155_420 => "OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT",
            8453 => "BASE_ROLLUP_ENDPOINT",
            84_532 => "BASE_SEPOLIA_ROLLUP_ENDPOINT",
            _ => return None,
        };

        env::var(env_var).ok()
    }
}

impl IFactory for Factory {
    fn create(&self, chain_id: ChainId) -> Result<Box<dyn IClient>, FactoryError> {
        let url = if let Some(override_url) = Self::get_rollup_endpoint_override(chain_id) {
            override_url
        } else {
            self.rpc_urls
                .get(&chain_id)
                .ok_or(Error::NoRpcUrl(chain_id))?
                .clone()
        };

        let client = HttpClientBuilder::default()
            .build(&url)
            .map_err(|err| Error::HttpClientBuilder(err.to_string()))?;
        Ok(Box::new(http::Client::new(client)))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use serial_test::serial;

    use super::*;

    #[test]
    #[serial]
    fn get_rollup_endpoint_override_optimism_mainnet() {
        let test_url = "http://test-optimism-mainnet.com";
        unsafe {
            env::set_var("OPTIMISM_ROLLUP_ENDPOINT", test_url);
        }

        let result = Factory::get_rollup_endpoint_override(10);

        unsafe {
            env::remove_var("OPTIMISM_ROLLUP_ENDPOINT");
        }
        assert_eq!(result, Some(test_url.to_string()));
    }

    #[test]
    #[serial]
    fn get_rollup_endpoint_override_optimism_sepolia() {
        let test_url = "http://test-optimism-sepolia.com";
        unsafe {
            env::set_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT", test_url);
        }

        let result = Factory::get_rollup_endpoint_override(11_155_420);

        unsafe {
            env::remove_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT");
        }
        assert_eq!(result, Some(test_url.to_string()));
    }

    #[test]
    #[serial]
    fn get_rollup_endpoint_override_base_mainnet() {
        let test_url = "http://test-base-mainnet.com";
        unsafe {
            env::set_var("BASE_ROLLUP_ENDPOINT", test_url);
        }

        let result = Factory::get_rollup_endpoint_override(8453);

        unsafe {
            env::remove_var("BASE_ROLLUP_ENDPOINT");
        }
        assert_eq!(result, Some(test_url.to_string()));
    }

    #[test]
    #[serial]
    fn get_rollup_endpoint_override_base_sepolia() {
        let test_url = "http://test-base-sepolia.com";
        unsafe {
            env::set_var("BASE_SEPOLIA_ROLLUP_ENDPOINT", test_url);
        }

        let result = Factory::get_rollup_endpoint_override(84_532);

        unsafe {
            env::remove_var("BASE_SEPOLIA_ROLLUP_ENDPOINT");
        }
        assert_eq!(result, Some(test_url.to_string()));
    }

    #[test]
    fn get_rollup_endpoint_override_unsupported_chain() {
        assert_eq!(Factory::get_rollup_endpoint_override(999), None);
        assert_eq!(Factory::get_rollup_endpoint_override(1), None);
        assert_eq!(Factory::get_rollup_endpoint_override(137), None);
    }

    #[test]
    #[serial]
    fn get_rollup_endpoint_override_no_env_var() {
        unsafe {
            env::remove_var("OPTIMISM_ROLLUP_ENDPOINT");
            env::remove_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT");
            env::remove_var("BASE_ROLLUP_ENDPOINT");
            env::remove_var("BASE_SEPOLIA_ROLLUP_ENDPOINT");
        }

        assert_eq!(Factory::get_rollup_endpoint_override(10), None);
        assert_eq!(Factory::get_rollup_endpoint_override(11_155_420), None);
        assert_eq!(Factory::get_rollup_endpoint_override(8453), None);
        assert_eq!(Factory::get_rollup_endpoint_override(84_532), None);
    }

    #[test]
    #[serial]
    fn create_with_rollup_override() {
        let test_url = "http://test-override.com";
        unsafe {
            env::set_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT", test_url);
        }

        let factory = Factory::new(HashMap::new());
        let result = factory.create(11_155_420);

        unsafe {
            env::remove_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT");
        }
        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn create_falls_back_to_rpc_urls() {
        unsafe {
            env::remove_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT");
        }

        let mut rpc_urls = HashMap::new();
        rpc_urls.insert(11_155_420, "http://fallback-url.com".to_string());

        let factory = Factory::new(rpc_urls);
        let result = factory.create(11_155_420);

        assert!(result.is_ok());
    }

    #[test]
    #[serial]
    fn create_fails_without_url_or_override() {
        unsafe {
            env::remove_var("OPTIMISM_ROLLUP_ENDPOINT");
            env::remove_var("OPTIMISM_SEPOLIA_ROLLUP_ENDPOINT");
            env::remove_var("BASE_ROLLUP_ENDPOINT");
            env::remove_var("BASE_SEPOLIA_ROLLUP_ENDPOINT");
        }

        // Verify no override is available
        assert_eq!(Factory::get_rollup_endpoint_override(11_155_420), None);

        let factory = Factory::new(HashMap::new());
        let result = factory.create(11_155_420);

        assert!(matches!(result, Err(FactoryError::Http(Error::NoRpcUrl(11_155_420)))));
    }

    #[test]
    #[serial]
    fn create_with_invalid_url_in_override() {
        unsafe {
            env::set_var("OPTIMISM_ROLLUP_ENDPOINT", "invalid-url");
        }

        let factory = Factory::new(HashMap::new());
        let result = factory.create(10);

        unsafe {
            env::remove_var("OPTIMISM_ROLLUP_ENDPOINT");
        }
        assert!(matches!(result, Err(FactoryError::Http(Error::HttpClientBuilder(_)))));
    }

    #[test]
    #[serial]
    fn create_with_empty_string_override() {
        unsafe {
            env::set_var("OPTIMISM_ROLLUP_ENDPOINT", "");
        }

        let factory = Factory::new(HashMap::new());
        let result = factory.create(10);

        unsafe {
            env::remove_var("OPTIMISM_ROLLUP_ENDPOINT");
        }
        assert!(matches!(result, Err(FactoryError::Http(Error::HttpClientBuilder(_)))));
    }

    #[test]
    #[serial]
    fn create_with_invalid_rpc_url_in_hashmap() {
        unsafe {
            env::remove_var("OPTIMISM_ROLLUP_ENDPOINT");
        }

        let mut rpc_urls = HashMap::new();
        rpc_urls.insert(10, "invalid-url".to_string());

        let factory = Factory::new(rpc_urls);
        let result = factory.create(10);

        assert!(matches!(result, Err(FactoryError::Http(Error::HttpClientBuilder(_)))));
    }
}
