use clap::Parser;
use reqwest::Url;

use crate::errors::Result;

#[derive(Clone, Debug, Parser)]
pub(crate) struct WebProofArgs {
    #[arg(long)]
    url: String,

    #[arg(long)]
    host: Option<String>,
}

pub(crate) async fn webproof_fetch(args: WebProofArgs) -> Result<()> {
    let server_args = to_server_proving_args(args);

    let presentation = Box::pin(web_prover::generate_web_proof(
        "127.0.0.1",
        7047,
        &server_args.domain,
        &server_args.host,
        server_args.port,
        &server_args.uri,
    ))
    .await
    .unwrap();
    println!("{presentation}");

    Ok(())
}

pub struct ServerProvingArgs {
    domain: String,
    host: String,
    port: u16,
    uri: String,
}
fn to_server_proving_args(args: WebProofArgs) -> ServerProvingArgs {
    let url = Url::parse(&args.url).expect("Failed to parse URL");

    let domain = url.host_str().expect("URL must have host").to_string();

    let port = url.port().unwrap_or_else(|| match url.scheme() {
        "https" => 443,
        _ => 80,
    });

    let uri = {
        let path = url.path();
        let query = url.query().map(|q| format!("?{q}")).unwrap_or_default();
        format!("{path}{query}")
    };

    let host = args.host.unwrap_or_else(|| domain.clone());

    ServerProvingArgs {
        domain,
        host,
        port,
        uri,
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

        let converted = to_server_proving_args(input_args);

        assert_eq!(converted.domain, "api.x.com");
        assert_eq!(converted.host, "127.0.0.1");
        assert_eq!(converted.port, 8080);
        assert_eq!(converted.uri, "/v1/followers?token=5daa4f53&uid=245");
    }

    #[test]
    fn test_convert_args_no_uri_params() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers".to_string(),
            ..default_args()
        };

        let converted = to_server_proving_args(input_args);

        assert_eq!(converted.uri, "/v1/followers");
    }
    #[test]
    fn test_convert_args_no_host_provided() {
        let input_args: WebProofArgs = WebProofArgs {
            url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".to_string(),
            host: None,
        };

        let converted = to_server_proving_args(input_args);

        assert_eq!(converted.host, "api.x.com");
    }
    fn default_args() -> WebProofArgs {
        WebProofArgs {
            url: "https://api.x.com:8080/v1/followers?token=5daa4f53&uid=245".to_string(),
            host: Option::from("127.0.0.1".to_string()),
        }
    }
}
