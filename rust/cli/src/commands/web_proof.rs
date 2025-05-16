use std::str::FromStr;

use clap::Parser;
use reqwest::Url;
use strum::EnumString;
use thiserror::Error;
use tracing::debug;
use web_prover::{
    Method, NotarizeParams, NotarizeParamsBuilder, NotarizeParamsBuilderError, NotaryConfig,
    NotaryConfigBuilder, NotaryConfigBuilderError, generate_web_proof,
};

#[derive(Debug, PartialEq, Eq, EnumString)]
enum Scheme {
    #[strum(serialize = "http")]
    Http,
    #[strum(serialize = "https")]
    Https,
}

const DEFAULT_NOTARY_URL: &str = "https://test-notary.vlayer.xyz/";
const DEFAULT_MAX_SENT_DATA: usize = 1 << 12;
const DEFAULT_MAX_RECV_DATA: usize = 1 << 14;

type Result<T> = std::result::Result<T, InputError>;

#[derive(Debug, Error)]
pub(crate) enum InputError {
    #[error("URL has no host: {0}")]
    MissingUrlHost(String),
    #[error("Invalid URL format: {0}")]
    InvalidUrlFormat(String),
    #[error("Invalid URL protocol: {0}")]
    InvalidUrlProtocol(String),
    #[error("Invalid header format: {0}")]
    InvalidHeaderFormat(String),
    #[error("Invalid notarize params: {0}")]
    NotarizeParams(#[from] NotarizeParamsBuilderError),
    #[error("Invalid notary config: {0}")]
    NotaryConfig(#[from] NotaryConfigBuilderError),
}

/// Generates a web-based proof for the specified request
#[derive(Clone, Parser, Debug)]
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

    /// HTTP method to use
    #[arg(short = 'X', long = "request", value_name = "METHOD")]
    method: Option<Method>,

    /// Additional header for the HTTP request (format: "Header-Name: Header-Value")
    #[arg(short = 'H', long = "header", value_name = "HEADER")]
    headers: Vec<String>,

    /// HTTP data to be sent with the request
    #[arg(short = 'd', long, value_name = "DATA")]
    data: Option<String>,

    #[arg(short = 'm', long, value_name = "MAX_SENT_DATA")]
    max_sent_data: Option<usize>,

    #[arg(short = 'M', long, value_name = "MAX_RECV_DATA")]
    max_recv_data: Option<usize>,
}

pub(crate) async fn webproof_fetch(args: WebProofArgs) -> anyhow::Result<()> {
    let server_args: NotarizeParams = args.try_into()?;

    debug!("notarizing...");

    let presentation = generate_web_proof(server_args).await?;

    println!("{presentation}");

    Ok(())
}

#[derive(Debug)]
struct ValidatedUrl {
    url: Url,
    host: String,
    scheme: Scheme,
    port: u16,
}

#[derive(Debug)]
struct ProvenUrl {
    host: String,
    port: u16,
}

impl ValidatedUrl {
    fn try_from_url(url_str: &str, allowed_schemes: &[Scheme]) -> Result<Self> {
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

fn parse_proven_url(url_str: &str) -> Result<ProvenUrl> {
    debug!("parsing url to notarize '{url_str}'");

    // Only https is allowed for proven urls as it does not make sense to prove http urls (not tls => no tlsn)
    let ValidatedUrl { host, port, .. } = ValidatedUrl::try_from_url(url_str, &[Scheme::Https])?;

    let url = ProvenUrl { host, port };

    debug!("proven url: {url:#?}");

    Ok(url)
}

fn parse_notary_url(url_str: &str) -> Result<NotaryConfig> {
    debug!("parsing notary url '{url_str}'");

    let ValidatedUrl {
        url,
        host,
        scheme,
        port,
    } = ValidatedUrl::try_from_url(url_str, &[Scheme::Https, Scheme::Http])?;

    let path_prefix = url.path().trim_matches('/');
    let enable_tls = scheme == Scheme::Https;

    let config = NotaryConfigBuilder::default()
        .host(host)
        .port(port)
        .path_prefix(path_prefix)
        .enable_tls(enable_tls)
        .build()?;

    debug!("notary config: {config:#?}");

    Ok(config)
}

fn parse_header(header_str: impl AsRef<str>) -> Result<(String, String)> {
    header_str
        .as_ref()
        .split_once(':')
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .ok_or(InputError::InvalidHeaderFormat(header_str.as_ref().to_string()))
        .and_then(|(key, value)| {
            (!key.is_empty() && !value.is_empty())
                .then_some((key, value))
                .ok_or(InputError::InvalidHeaderFormat(header_str.as_ref().to_string()))
        })
}

impl TryFrom<WebProofArgs> for NotarizeParams {
    type Error = InputError;

    fn try_from(value: WebProofArgs) -> Result<Self> {
        let ProvenUrl { host, port } = parse_proven_url(&value.url)?;

        // If host is not provided fallback to host extracted from url
        let fallback_host = value.host.unwrap_or(host.clone());

        debug!("fallback host for notarizing '{fallback_host}'");

        let max_sent_data = value.max_sent_data.unwrap_or(DEFAULT_MAX_SENT_DATA);
        let max_recv_data = value.max_recv_data.unwrap_or(DEFAULT_MAX_RECV_DATA);

        let method = value.method.unwrap_or_else(|| {
            if value.data.is_some() {
                Method::POST
            } else {
                Method::GET
            }
        });

        debug!("HTTP method: {method}");

        let headers = value
            .headers
            .iter()
            .map(parse_header)
            .collect::<Result<Vec<(String, String)>>>()?;

        debug!("headers: {headers:#?}");

        let notary_url = value.notary.unwrap_or(DEFAULT_NOTARY_URL.to_string());
        let notary_config = parse_notary_url(&notary_url)?;

        let mut notarize_params_builder = NotarizeParamsBuilder::default();
        notarize_params_builder
            .notary_config(notary_config)
            .server_domain(host)
            .server_host(fallback_host)
            .server_port(port)
            .max_sent_data(max_sent_data)
            .max_recv_data(max_recv_data)
            .uri(value.url)
            .headers(headers)
            .method(method);

        if let Some(body) = value.data {
            notarize_params_builder.body(body);
        }

        Ok(notarize_params_builder.build()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_args() {
        let input_args = WebProofArgs {
            headers: vec!["Authorization: Basic 1234".into(), "X-Api-Key: 5678".into()],
            data: Some("example body data".into()),
            max_sent_data: Some(100),
            max_recv_data: Some(100),
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert_eq!(converted.server_domain, "api.x.com");
        assert_eq!(converted.server_host, "127.0.0.1");
        assert_eq!(converted.server_port, 8080);
        assert_eq!(converted.uri, "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245");
        assert_eq!(converted.notary_config.host, "notary.pse.dev");
        assert_eq!(converted.notary_config.port, 3030);
        assert_eq!(converted.notary_config.path_prefix, "v0.1.0-alpha.9");
        assert_eq!(converted.headers.get("Authorization"), Some(&"Basic 1234".to_string()));
        assert_eq!(converted.headers.get("X-Api-Key"), Some(&"5678".to_string()));
        assert_eq!(converted.body, "example body data".as_bytes());
        assert_eq!(converted.max_sent_data, 100);
        assert_eq!(converted.max_recv_data, 100);
    }

    #[test]
    fn test_parse_headers() {
        let input_args: WebProofArgs = WebProofArgs {
            headers: vec!["Auth:oriza:tion: Basic 1234".into(), "X-Api-Key: 5678".into()],
            ..Default::default()
        };
        let converted: NotarizeParams = input_args.try_into().unwrap();
        assert_eq!(converted.headers.get("Auth"), Some(&"oriza:tion: Basic 1234".to_string()));
        assert_eq!(converted.headers.get("X-Api-Key"), Some(&"5678".to_string()));
    }

    #[test]
    fn test_default_notary_args() {
        let input_args = WebProofArgs {
            notary: None,
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert_eq!(converted.notary_config.host, "test-notary.vlayer.xyz");
        assert_eq!(converted.notary_config.port, 443);
        assert_eq!(converted.notary_config.path_prefix, "");
        assert_eq!(converted.max_sent_data, DEFAULT_MAX_SENT_DATA);
        assert_eq!(converted.max_recv_data, DEFAULT_MAX_RECV_DATA);
        assert!(converted.notary_config.enable_tls);
    }

    #[test]
    fn test_default_method_no_data() {
        let input_args = WebProofArgs {
            data: None,
            ..Default::default()
        };
        let converted: NotarizeParams = input_args.try_into().unwrap();
        assert_eq!(converted.method, Method::GET);
    }

    #[test]
    fn test_default_method_with_data() {
        let input_args = WebProofArgs {
            data: Some("something".to_string()),
            ..Default::default()
        };
        let converted: NotarizeParams = input_args.try_into().unwrap();
        assert_eq!(converted.method, Method::POST);
    }

    #[test]
    fn test_trim_slashes_in_notary_path() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert_eq!(converted.notary_config.host, "notary.vlayer.xyz");
        assert_eq!(converted.notary_config.path_prefix, "path/to/api");
    }

    #[test]
    fn test_set_notary_tls_https() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert!(converted.notary_config.enable_tls);
    }

    #[test]
    fn test_set_notary_tls_http() {
        let input_args = WebProofArgs {
            notary: Some("http://notary.vlayer.xyz/path/to/api/".into()),
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert!(!converted.notary_config.enable_tls);
    }

    #[test]
    fn test_convert_args_no_uri_params() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers".to_string(),
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert_eq!(converted.uri, "https://api.x.com:8080/v1/followers");
    }
    #[test]
    fn test_convert_args_no_host_provided() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".to_string(),
            host: None,
            ..Default::default()
        };

        let converted: NotarizeParams = input_args.try_into().unwrap();

        assert_eq!(converted.server_host, "api.x.com");
    }

    #[test]
    fn test_invalid_proven_url_error() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "invalid-url".to_string(),
            ..Default::default()
        };

        let result: Result<NotarizeParams> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlFormat("invalid-url".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_notary_url_error() {
        let input_args: WebProofArgs = WebProofArgs {
            notary: Some("invalid-url".to_string()),
            ..Default::default()
        };

        let result: Result<NotarizeParams> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlFormat("invalid-url".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_proven_url_protocol_error() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "xyz:///path/to/resource".to_string(),
            ..Default::default()
        };

        let result: Result<NotarizeParams> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlProtocol("xyz".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_notary_url_protocol_error() {
        let input_args: WebProofArgs = WebProofArgs {
            notary: Some("htp://notary.vlayer.xyz/path/to/api/".into()),
            ..Default::default()
        };

        let result: Result<NotarizeParams> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidUrlProtocol("htp".to_string()).to_string()
        );
    }

    #[test]
    fn test_invalid_header_format_error() {
        let input_args: WebProofArgs = WebProofArgs {
            headers: vec!["Authorization".into()],
            ..Default::default()
        };

        let result: Result<NotarizeParams> = input_args.try_into();
        assert_eq!(
            format!("{}", result.unwrap_err()),
            InputError::InvalidHeaderFormat("Authorization".to_string()).to_string()
        );
    }

    #[test]
    fn test_parse_header_success() {
        let success = |(key, value): (&str, &str), input: &str| {
            assert_eq!((key.to_string(), value.to_string()), parse_header(input).unwrap());
        };
        success(("Authorization", "Bearer 1234"), "Authorization: Bearer 1234");
        success(("Authorization", "Bearer 1234"), "Authorization:Bearer 1234");
        success(("Authorization", "Bearer 1234"), "Authorization: Bearer 1234  ");
        success(("Authorization", "Bearer 1234"), "  Authorization : Bearer 1234  ");
        success(("Authorization", "Bearer 1234 :"), "  Authorization : Bearer 1234 :  ");
    }

    #[test]
    fn test_parse_header_failure() {
        let fail = |input: &str| {
            let err = parse_header(input).unwrap_err();
            assert!(matches!(err, InputError::InvalidHeaderFormat(..)));
        };
        fail("");
        fail(":");
        fail("    :     ");
        fail("Authorization");
        fail(" Authorization  Bearer  ");
    }

    impl Default for WebProofArgs {
        fn default() -> Self {
            Self {
                url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".into(),
                host: Some("127.0.0.1".into()),
                notary: Some("https://notary.pse.dev:3030/v0.1.0-alpha.9".into()),
                method: None,
                headers: vec![],
                data: None,
                max_sent_data: Some(DEFAULT_MAX_SENT_DATA),
                max_recv_data: Some(DEFAULT_MAX_RECV_DATA),
            }
        }
    }
}
