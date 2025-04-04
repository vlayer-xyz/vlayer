mod notarize;
mod presentation;
mod verify;

use std::collections::HashMap;

use constcat::concat;
pub use notarize::notarize;
use notarize::NotarizeParamsBuilder;
pub use presentation::create_presentation_with_redaction;
use serde_json::Value;
use tlsn_core::transcript::Transcript;
use utils::range::RangeSet;
pub use verify::verify_presentation;

pub use crate::notarize::{NotaryConfig, RedactionConfigFn};

pub const TLSN_VERSION: &str = "0.1.0-alpha.8";
pub const TLSN_VERSION_WITH_V_PREFIX: &str = concat!("v", TLSN_VERSION);

pub struct RedactionConfig {
    pub sent: RangeSet<usize>,
    pub recv: RangeSet<usize>,
}

pub fn no_redaction_config(transcript: &Transcript) -> RedactionConfig {
    RedactionConfig {
        sent: RangeSet::from(0..transcript.sent().len()),
        recv: RangeSet::from(0..transcript.received().len()),
    }
}

pub async fn generate_web_proof_with_redaction(
    notary_config: NotaryConfig,
    server_domain: &str,
    server_host: &str,
    server_port: u16,
    uri: &str,
    headers: HashMap<String, String>,
    redaction_config_fn: RedactionConfigFn,
) -> Result<String, Box<dyn std::error::Error>> {
    let params = NotarizeParamsBuilder::default()
        .notary_config(notary_config.clone())
        .server_domain(server_domain.to_string())
        .server_host(server_host.to_string())
        .server_port(server_port)
        .uri(uri.to_string())
        .headers(headers)
        .redaction_config_fn(redaction_config_fn)
        .build()?;

    let (attestation, secrets, redaction_config) = notarize(params).await?;
    let presentation =
        create_presentation_with_redaction(&attestation, &secrets, &redaction_config)?;
    let encoded_presentation = hex::encode(bincode::serialize(&presentation).unwrap());

    let json_response = to_json(&encoded_presentation, &notary_config.host, notary_config.port);

    Ok(serde_json::to_string(&json_response)?)
}

pub async fn generate_web_proof(
    notary_config: NotaryConfig,
    server_domain: &str,
    server_host: &str,
    server_port: u16,
    uri: &str,
    headers: HashMap<String, String>,
) -> Result<String, Box<dyn std::error::Error>> {
    generate_web_proof_with_redaction(
        notary_config,
        server_domain,
        server_host,
        server_port,
        uri,
        headers,
        no_redaction_config,
    )
    .await
}

fn to_json(encoded_presentation: &str, notary_host: &str, notary_port: u16) -> Value {
    let notary_url = format!("https://{notary_host}:{notary_port}");

    let presentation_json = serde_json::json!({
        "presentationJson": {
            "version": TLSN_VERSION,
            "data": encoded_presentation,
            "meta": {
                "notaryUrl": notary_url,
                "websocketProxyUrl": "",
            },
        }
    });
    presentation_json
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_json_structure() {
        let json = to_json("48656c6c6f20576f726c64", "127.0.0.1", 7047);

        assert!(json.is_object());
        assert_eq!(json["presentationJson"]["version"], TLSN_VERSION);
        assert_eq!(json["presentationJson"]["data"], "48656c6c6f20576f726c64");
        assert_eq!(json["presentationJson"]["meta"]["notaryUrl"], "https://127.0.0.1:7047");
        assert_eq!(json["presentationJson"]["meta"]["websocketProxyUrl"], "");
    }
}
