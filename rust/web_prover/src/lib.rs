mod notarize;
mod params;
mod presentation;
mod verify;

use anyhow::Result;
use constcat::concat;
pub use notarize::notarize;
pub use params::{
    Method, NotarizeParams, NotarizeParamsBuilder, NotarizeParamsBuilderError, NotaryConfig,
    NotaryConfigBuilder, NotaryConfigBuilderError, RedactionConfigFn,
};
pub use presentation::create_presentation_with_redaction;
use rangeset::RangeSet;
use serde_json::Value;
use tlsn_core::transcript::Transcript;
pub use verify::verify_presentation;

pub const TLSN_VERSION: &str = "0.1.0-alpha.10";
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

pub async fn generate_web_proof(notarize_params: NotarizeParams) -> Result<String> {
    let (attestation, secrets, redaction_config) = notarize(notarize_params.clone()).await?;
    let presentation =
        create_presentation_with_redaction(&attestation, &secrets, &redaction_config)?;
    let encoded_presentation = hex::encode(bincode::serialize(&presentation)?);

    let notary_config = notarize_params.notary_config;

    let json_response = to_json(&encoded_presentation, &notary_config.host, notary_config.port);

    Ok(serde_json::to_string(&json_response)?)
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
