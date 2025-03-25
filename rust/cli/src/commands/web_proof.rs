use anyhow::{anyhow, Context};
use clap::Parser;
use derive_new::new;
use reqwest::Url;
use web_prover::generate_web_proof;

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
        default_value = "https://test-notary.vlayer.xyz",
        value_name = "NOTARY_URL"
    )]
    notary: Option<String>,
}

pub(crate) async fn webproof_fetch(args: WebProofArgs) -> anyhow::Result<()> {
    let server_args: ServerProvingArgs = args.try_into()?;

    let presentation = generate_web_proof(
        &server_args.notary_host,
        server_args.notary_port,
        &server_args.notary_path,
        server_args.notary_tls,
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

#[derive(new)]
pub struct ServerProvingArgs {
    domain: String,
    host: String,
    port: u16,
    uri: String,
    notary_host: String,
    notary_port: u16,
    notary_path: String,
    notary_tls: bool,
}
impl TryFrom<WebProofArgs> for ServerProvingArgs {
    type Error = anyhow::Error;

    fn try_from(value: WebProofArgs) -> Result<Self, Self::Error> {
        let url = Url::parse(&value.url)?;

        let domain = url.host_str().context("Url has no host")?.to_string();

        let port = Self::extract_port(&url);

        let uri = {
            let path = url.path();
            let query = url.query().map(|q| format!("?{q}")).unwrap_or_default();
            format!("{path}{query}")
        };

        let host = value.host.unwrap_or(domain.clone());

        let (notary_host, notary_port, notary_path, notary_tls) =
            if let Some(notary_url) = value.notary {
                let notary_url = Url::parse(&notary_url)?;
                let notary_host = notary_url
                    .host_str()
                    .context("Notary URL has no host")?
                    .to_string();
                let notary_port = Self::extract_port(&notary_url);
                let notary_path = notary_url
                    .path()
                    .trim_start_matches('/')
                    .trim_end_matches('/')
                    .to_string();
                let notary_tls = notary_url.scheme() == "https";
                (notary_host, notary_port, notary_path, notary_tls)
            } else {
                ("test-notary.vlayer.xyz".into(), 443, "".to_string(), true)
            };

        Ok(ServerProvingArgs::new(
            domain,
            host,
            port,
            uri,
            notary_host,
            notary_port,
            notary_path,
            notary_tls,
        ))
    }
}

impl ServerProvingArgs {
    fn extract_port(url: &Url) -> u16 {
        let port = url.port().unwrap_or_else(|| match url.scheme() {
            "https" => 443,
            _ => 80,
        });
        port
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
        assert_eq!(converted.notary_host, "notary.pse.dev");
        assert_eq!(converted.notary_port, 3030);
        assert_eq!(converted.notary_path, "v0.1.0-alpha.8");
    }

    #[test]
    fn test_default_notary_args() {
        let input_args = WebProofArgs {
            notary: None,
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_host, "test-notary.vlayer.xyz");
        assert_eq!(converted.notary_port, 443);
        assert_eq!(converted.notary_path, "");
        assert_eq!(converted.notary_tls, true);
    }

    #[test]
    fn test_trim_slashes_in_notary_path() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_host, "notary.vlayer.xyz");
        assert_eq!(converted.notary_path, "path/to/api");
    }

    #[test]
    fn test_set_notary_tls_https() {
        let input_args = WebProofArgs {
            notary: Some("https://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_tls, true);
    }

    #[test]
    fn test_set_notary_tls_http() {
        let input_args = WebProofArgs {
            notary: Some("http://notary.vlayer.xyz/path/to/api/".into()),
            ..WebProofArgs::default()
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.notary_tls, false);
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
