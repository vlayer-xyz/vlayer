use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use derive_new::new;
use reqwest::Url;
use strum::EnumString;
use thiserror::Error;
use web_prover::{generate_web_proof, NotaryConfig};

#[derive(Debug, PartialEq, Eq, EnumString)]
pub enum AllowedScheme {
    #[strum(serialize = "http")]
    Http,
    #[strum(serialize = "https")]
    Https,
}

const DEFAULT_NOTARY_URL: &str = "https://test-notary.vlayer.xyz";

#[derive(Debug, Error)]
pub enum WebProofInputError {
    #[error("Proven URL has no host")]
    MissingProvenUrlHost,
    #[error("Notary URL has no host")]
    MissingNotaryUrlHost,
    #[error("Invalid proven URL format")]
    InvalidProvenUrl,
    #[error("Invalid notary URL format")]
    InvalidNotaryUrl,
    #[error("Invalid notary  URL protocol")]
    InvalidNotaryUrlProtocol,
    #[error("Invalid proven URL protocol")]
    InvalidProvenUrlProtocol,
}

/// Generates a web-based proof for the specified request
#[derive(Clone, Debug, Parser)]
pub(crate) struct WebProofArgs {
    /// Full URL of the request to notarize
    #[arg(long)]
    url: String,

    /// Optional host address, if different from the domain provided in URL
    #[arg(long)]
    host: Option<String>,

    /// Full notary URL
    #[arg(
        long,
        default_value = DEFAULT_NOTARY_URL,
        value_name = "NOTARY_URL"
    )]
    notary: Option<String>,
}

pub(crate) async fn webproof_fetch(args: WebProofArgs) -> anyhow::Result<()> {
    let server_args: ServerProvingArgs = args.try_into()?;

    let presentation = generate_web_proof(
        server_args.notary_config,
        &server_args.domain,
        &server_args.host,
        server_args.port,
        &server_args.uri,
    )
    .await
    .map_err(|e| anyhow!("{e}"))?;
    println!("{presentation}");

    Ok(())
}

#[derive(new, Debug)]
pub struct ServerProvingArgs {
    domain: String,
    host: String,
    port: u16,
    uri: String,
    notary_config: NotaryConfig,
}
impl ServerProvingArgs {
    fn parse_proven_url(
        url_str: &str,
    ) -> Result<(String, String, u16, String), WebProofInputError> {
        let url = Self::validate_proven_url(url_str)?;

        let domain = url.host_str().unwrap().to_string();
        let port = Self::extract_port(&url);
        let uri = {
            let path = url.path();
            let query = url.query().map(|q| format!("?{q}")).unwrap_or_default();
            format!("{path}{query}")
        };

        Ok((domain.clone(), uri, port, domain))
    }

    fn extract_port(url: &Url) -> u16 {
        let port = url.port().unwrap_or_else(|| match url.scheme() {
            "https" => 443,
            _ => 80,
        });
        port
    }

    fn validate_proven_url(url_str: &str) -> Result<Url, WebProofInputError> {
        let url = Url::parse(url_str).map_err(|_| WebProofInputError::InvalidProvenUrl)?;
        AllowedScheme::from_str(url.scheme())
            .map_err(|_| WebProofInputError::InvalidProvenUrlProtocol)?;
        url.host_str()
            .ok_or(WebProofInputError::MissingProvenUrlHost)?;
        Ok(url)
    }

    fn validate_notary_url(url_str: &str) -> Result<Url, WebProofInputError> {
        let url = Url::parse(url_str).map_err(|_| WebProofInputError::InvalidNotaryUrl)?;
        AllowedScheme::from_str(url.scheme())
            .map_err(|_| WebProofInputError::InvalidNotaryUrlProtocol)?;
        url.host_str()
            .ok_or(WebProofInputError::MissingNotaryUrlHost)?;
        Ok(url)
    }

    fn parse_notary_url(url_str: &str) -> Result<(String, u16, String, bool), WebProofInputError> {
        let url = Self::validate_notary_url(url_str)?;

        let notary_host = url.host_str().unwrap().to_string();
        let notary_port = Self::extract_port(&url);
        let notary_path = url
            .path()
            .trim_start_matches('/')
            .trim_end_matches('/')
            .to_string();
        let notary_tls = url.scheme() == "https";

        Ok((notary_host, notary_port, notary_path, notary_tls))
    }
}
impl TryFrom<WebProofArgs> for ServerProvingArgs {
    type Error = WebProofInputError;

    fn try_from(value: WebProofArgs) -> Result<Self, Self::Error> {
        let (domain, uri, port, default_host) = Self::parse_proven_url(&value.url)?;
        let host = value.host.unwrap_or(default_host);

        let notary_config = if let Some(notary_url) = value.notary {
            let (notary_host, notary_port, notary_path, notary_tls) =
                Self::parse_notary_url(&notary_url)?;
            NotaryConfig::new(notary_host, notary_port, notary_path, notary_tls)
        } else {
            {
                let (notary_host, _, _, _) = Self::parse_notary_url(DEFAULT_NOTARY_URL)?;
                NotaryConfig::new(notary_host, 443, "".to_string(), true)
            }
        };

        Ok(ServerProvingArgs::new(domain, host, port, uri, notary_config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_args() {
        let input_args = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".to_string(),
            host: Some("127.0.0.1".into()),
            notary: Some("https://notary.pse.dev:3030/v0.1.0-alpha.8".into()),
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.domain, "api.x.com");
        assert_eq!(converted.host, "127.0.0.1");
        assert_eq!(converted.port, 8080);
        assert_eq!(converted.uri, "/v1/followers?token=5daa4f53&uid=245");
        assert_eq!(converted.notary_config.host, "notary.pse.dev");
        assert_eq!(converted.notary_config.port, 3030);
        assert_eq!(converted.notary_config.path, "v0.1.0-alpha.8");
    }

    #[test]
    fn test_default_notary_args() {
        let input_args = WebProofArgs {
            notary: None,
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_config.host, "test-notary.vlayer.xyz");
        assert_eq!(converted.notary_config.port, 443);
        assert_eq!(converted.notary_config.path, "");
        assert!(converted.notary_config.tls);
    }

    #[test]
    fn test_trim_slashes_in_notary_path() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_config.host, "notary.vlayer.xyz");
        assert_eq!(converted.notary_config.path, "path/to/api");
    }

    #[test]
    fn test_set_notary_tls_https() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert!(converted.notary_config.tls);
    }

    #[test]
    fn test_set_notary_tls_http() {
        let input_args = WebProofArgs {
            notary: Some("http://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert!(!converted.notary_config.tls);
    }

    #[test]
    fn test_convert_args_no_uri_params() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers".to_string(),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.uri, "/v1/followers");
    }
    #[test]
    fn test_convert_args_no_host_provided() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".to_string(),
            host: None,
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.host, "api.x.com");
    }

    #[test]
    fn test_invalid_proven_url_error() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "invalid-url".to_string(),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            WebProofInputError::InvalidProvenUrl.to_string()
        );
    }

    #[test]
    fn test_invalid_notary_url_error() {
        let input_args: WebProofArgs = WebProofArgs {
            notary: Some("invalid-url".to_string()),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            WebProofInputError::InvalidNotaryUrl.to_string()
        );
    }

    #[test]
    fn test_invalid_proven_url_protocol_error() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "htp:///path/to/resource".to_string(),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            WebProofInputError::InvalidProvenUrlProtocol.to_string()
        );
    }

    #[test]
    fn test_invalid_notary_url_protocol_error() {
        let input_args: WebProofArgs = WebProofArgs {
            notary: Some("htp://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            WebProofInputError::InvalidNotaryUrlProtocol.to_string()
        );
    }

    impl Default for WebProofArgs {
        fn default() -> Self {
            Self {
                url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".into(),
                host: Some("127.0.0.1".into()),
                notary: Some("https://notary.pse.dev:3030/v0.1.0-alpha.8".into()),
            }
        }
    }
}
