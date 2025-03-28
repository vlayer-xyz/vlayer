use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;
use clap::Parser;
use derive_new::new;
use reqwest::Url;
use strum::EnumString;
use thiserror::Error;
use web_prover::{generate_web_proof, NotaryConfig};

#[derive(Debug, PartialEq, Eq, EnumString)]
pub enum Scheme {
    #[strum(serialize = "http")]
    Http,
    #[strum(serialize = "https")]
    Https,
}

const DEFAULT_NOTARY_URL: &str = "https://test-notary.vlayer.xyz/v0.1.0-alpha.8";

#[derive(Debug, Error)]
pub enum InputError {
    #[error("URL has no host: {0}")]
    MissingUrlHost(String),
    #[error("Invalid URL format: {0}")]
    InvalidUrlFormat(String),
    #[error("Invalid URL protocol: {0}")]
    InvalidUrlProtocol(String),
    #[error("Invalid header format: {0}")]
    InvalidHeaderFormat(String),
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

    /// Additional headers for the HTTP request (format: "Header-Name: Header-Value")
    #[arg(short = 'H', long, value_name = "HEADER")]
    headers: Vec<String>,
}

pub(crate) async fn webproof_fetch(args: WebProofArgs) -> anyhow::Result<()> {
    let server_args: ServerProvingArgs = args.try_into()?;

    let presentation = generate_web_proof(
        server_args.notary_config,
        &server_args.domain,
        &server_args.host,
        server_args.port,
        &server_args.uri,
        server_args.headers,
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
    headers: HashMap<String, String>,
}

struct ValidatedUrl {
    url: Url,
    host: String,
    scheme: Scheme,
    port: u16,
}

struct ProvenUrl {
    host: String,
    port: u16,
    uri: String,
}

impl ValidatedUrl {
    fn try_from_url(url_str: &str, allowed_schemes: &[Scheme]) -> Result<Self, InputError> {
        let url =
            Url::parse(url_str).map_err(|_| InputError::InvalidUrlFormat(url_str.to_string()))?;
        let scheme = Scheme::from_str(url.scheme())
            .map_err(|_| InputError::InvalidUrlProtocol(url.scheme().to_string()))?;
        if !allowed_schemes.contains(&scheme) {
            return Err(InputError::InvalidUrlProtocol(url.scheme().to_string()));
        }
        let host = url
            .host_str()
            .ok_or_else(|| InputError::MissingUrlHost(url_str.to_string()))?
            .to_string();
        let port = url.port().unwrap_or(match scheme {
            Scheme::Https => 443,
            Scheme::Http => 80,
        });

        Ok(Self {
            url,
            host,
            scheme,
            port,
        })
    }
}

fn parse_proven_url(url_str: &str) -> Result<ProvenUrl, InputError> {
    //Only https is allowed for proven urls as it does not make sense to prove http urls (not tls => no tlsn)

    let ValidatedUrl {
        url, host, port, ..
    } = ValidatedUrl::try_from_url(url_str, &[Scheme::Https])?;
    let uri = {
        let path = url.path();
        let query = url.query().map(|q| format!("?{q}")).unwrap_or_default();
        format!("{path}{query}")
    };

    Ok(ProvenUrl { host, port, uri })
}

fn parse_notary_url(url_str: &str) -> Result<NotaryConfig, InputError> {
    let ValidatedUrl {
        url,
        host,
        scheme,
        port,
    } = ValidatedUrl::try_from_url(url_str, &[Scheme::Https, Scheme::Http])?;

    let path_prefix = url
        .path()
        .trim_start_matches('/')
        .trim_end_matches('/')
        .to_string();
    let enable_tls = scheme == Scheme::Https;

    Ok(NotaryConfig::new(host, port, path_prefix, enable_tls))
}

impl TryFrom<WebProofArgs> for ServerProvingArgs {
    type Error = InputError;

    fn try_from(value: WebProofArgs) -> Result<Self, Self::Error> {
        let ProvenUrl {
            host: urlhost,
            port,
            uri,
        } = parse_proven_url(&value.url)?;
        //If host is provided fallback to host extracted from url, otherwise use the host from the url
        let host = value.host.unwrap_or(urlhost.clone());
        let headers: Result<HashMap<String, String>, InputError> = value
            .headers
            .iter()
            .map(|header| {
                let mut parts = header.splitn(2, ':');
                let key = parts
                    .next()
                    .ok_or_else(|| InputError::InvalidHeaderFormat(header.clone()))?
                    .trim()
                    .to_string();
                let value = parts
                    .next()
                    .ok_or_else(|| InputError::InvalidHeaderFormat(header.clone()))?
                    .trim()
                    .to_string();
                Ok((key, value))
            })
            .collect();

        let notary_config = if let Some(notary_url) = value.notary {
            parse_notary_url(&notary_url)?
        } else {
            parse_notary_url(DEFAULT_NOTARY_URL)?
        };

        Ok(ServerProvingArgs::new(urlhost, host, port, uri, notary_config, headers?))
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
            headers: vec!["Authorization: Basic 1234".into(), "X-Api-Key: 5678".into()],
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.domain, "api.x.com");
        assert_eq!(converted.host, "127.0.0.1");
        assert_eq!(converted.port, 8080);
        assert_eq!(converted.uri, "/v1/followers?token=5daa4f53&uid=245");
        assert_eq!(converted.notary_config.host, "notary.pse.dev");
        assert_eq!(converted.notary_config.port, 3030);
        assert_eq!(converted.notary_config.path_prefix, "v0.1.0-alpha.8");
        assert_eq!(converted.headers.get("Authorization"), Some(&"Basic 1234".to_string()));
        assert_eq!(converted.headers.get("X-Api-Key"), Some(&"5678".to_string()));
    }

    #[test]
    fn test_parse_headers() {
        let input_args: WebProofArgs = WebProofArgs {
            headers: vec!["Auth:oriza:tion: Basic 1234".into(), "X-Api-Key: 5678".into()],
            ..WebProofArgs::default()
        };
        let converted: ServerProvingArgs = input_args.try_into().unwrap();
        assert_eq!(converted.headers.get("Auth"), Some(&"oriza:tion: Basic 1234".to_string()));
        assert_eq!(converted.headers.get("X-Api-Key"), Some(&"5678".to_string()));
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
        assert_eq!(converted.notary_config.path_prefix, "v0.1.0-alpha.8");
        assert!(converted.notary_config.enable_tls);
    }

    #[test]
    fn test_trim_slashes_in_notary_path() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_config.host, "notary.vlayer.xyz");
        assert_eq!(converted.notary_config.path_prefix, "path/to/api");
    }

    #[test]
    fn test_set_notary_tls_https() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert!(converted.notary_config.enable_tls);
    }

    #[test]
    fn test_set_notary_tls_http() {
        let input_args = WebProofArgs {
            notary: Some("http://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert!(!converted.notary_config.enable_tls);
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
            ..Default::default()
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
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlFormat("invalid-url".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_notary_url_error() {
        let input_args: WebProofArgs = WebProofArgs {
            notary: Some("invalid-url".to_string()),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlFormat("invalid-url".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_proven_url_protocol_error() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "xyz:///path/to/resource".to_string(),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlProtocol("xyz".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_notary_url_protocol_error() {
        let input_args: WebProofArgs = WebProofArgs {
            notary: Some("htp://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlProtocol("htp".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_header_format_error() {
        let input_args: WebProofArgs = WebProofArgs {
            headers: vec!["Authorization".into()],
            ..WebProofArgs::default()
        };

        let result: Result<ServerProvingArgs, _> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidHeaderFormat("Authorization".to_string()).to_string()
        );
    }

    impl Default for WebProofArgs {
        fn default() -> Self {
            Self {
                url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".into(),
                host: Some("127.0.0.1".into()),
                notary: Some("https://notary.pse.dev:3030/v0.1.0-alpha.8".into()),
                headers: vec![],
            }
        }
    }
}
