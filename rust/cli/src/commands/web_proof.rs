use anyhow::{Context, anyhow};
use clap::Parser;
use derive_new::new;
use reqwest::Url;
use web_prover::generate_web_proof;

/// Generates a web-based proof for the specified request
#[derive(Clone, Debug, Parser)]
pub(crate) struct WebProofArgs {
    #[arg(long)]
    url: String,

    #[arg(long)]
    host: Option<String>,
}

pub(crate) async fn webproof_fetch(args: WebProofArgs) -> anyhow::Result<()> {
    let server_args: ServerProvingArgs = args.try_into()?;

    let presentation = generate_web_proof(
        "127.0.0.1",
        7047,
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
}
impl TryFrom<WebProofArgs> for ServerProvingArgs {
    type Error = anyhow::Error;

    fn try_from(value: WebProofArgs) -> Result<Self, Self::Error> {
        let url = Url::parse(&value.url)?;

        let domain = url.host_str().context("Url has no host")?.to_string();

        let port = url.port().unwrap_or_else(|| match url.scheme() {
            "https" => 443,
            _ => 80,
        });

        let uri = {
            let path = url.path();
            let query = url.query().map(|q| format!("?{q}")).unwrap_or_default();
            format!("{path}{query}")
        };

        let host = value.host.unwrap_or(domain.clone());

        Ok(ServerProvingArgs::new(domain, host, port, uri))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_args() {
        let input_args = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".to_string(),
            host: Option::from("127.0.0.1".to_string()),
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.domain, "api.x.com");
        assert_eq!(converted.host, "127.0.0.1");
        assert_eq!(converted.port, 8080);
        assert_eq!(converted.uri, "/v1/followers?token=5daa4f53&uid=245");
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
        };

        let converted: ServerProvingArgs = input_args.try_into().unwrap();

        assert_eq!(converted.host, "api.x.com");
    }

    impl Default for WebProofArgs {
        fn default() -> Self {
            Self {
                url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".into(),
                host: Some("127.0.0.1".into()),
            }
        }
    }
}
