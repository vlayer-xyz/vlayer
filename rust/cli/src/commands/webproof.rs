#[derive(Clone, Debug, Parser)]
pub(crate) struct WebProofArgs {
    /// Url to notarize
    #[arg(long)]
    url: Option<String>,
    /// Optional server host
    #[arg(long)]
    host: Option<String>,
}

pub(crate) async fn webproof_fetch(
    _args: WebProofArgs,
    webproof: WebProof,
) -> crate::errors::Result<()> {
    let presentation = webproof
        .fetch(
            "lotr-api.online",
            "127.0.0.1",
            3011,
            "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        )
        .await;
    println!("{presentation}");

    Ok(())
}

use clap::Parser;
use web_prover::generate_web_proof;

#[cfg_attr(test, faux::create)]
pub struct WebProof {}
#[cfg_attr(test, faux::methods)]
impl WebProof {
    pub const fn new() -> Self {
        WebProof {}
    }

    pub async fn fetch(
        &self,
        server_domain: &str,
        server_host: &str,
        server_port: u16,
        server_uri: &str,
    ) -> String {
        Box::pin(generate_web_proof(
            "127.0.0.1",
            7047,
            server_domain,
            server_host,
            server_port,
            server_uri,
        ))
        .await
        .unwrap()
    }
}

#[tokio::test]
async fn test_fetch_args() {
    use faux::when;

    let mut webproof_ext = WebProof::faux();
    when!(webproof_ext.fetch("api.x.com", "127.0.0.1", 8080, "some_uri"))
        .then_return("proof".to_string());

    let args = WebProofArgs {
        url: Some("https://api.x.com:8080/some_uri".to_string()),
        host: Some("127.0.0.1".to_string()),
    };

    webproof_fetch(args, webproof_ext).await.unwrap();
}
