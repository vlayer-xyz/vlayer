mod notarize;
mod presentation;
mod verify;

pub use notarize::notarize;
pub use presentation::create_presentation;
use serde_json::Value;
pub use verify::verify_presentation;

pub use crate::notarize::NotaryConfig;

pub async fn generate_web_proof(
    notary_config: NotaryConfig,
    server_domain: &str,
    server_host: &str,
    server_port: u16,
    uri: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (attestation, secrets) =
        notarize(notary_config.clone(), server_domain, server_host, server_port, uri).await?;
    let presentation = create_presentation(attestation, secrets).await?;
    let encoded_presentation = hex::encode(bincode::serialize(&presentation).unwrap());

    let json_response = to_json(&encoded_presentation, &notary_config.host, notary_config.port);

    Ok(serde_json::to_string(&json_response)?)
}

fn to_json(encoded_presentation: &str, notary_host: &str, notary_port: u16) -> Value {
    let notary_url = format!("https://{notary_host}:{notary_port}");

    let presentation_json = serde_json::json!({
        "presentationJson": {
            "version": "0.1.0-alpha.8",
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
    use super::to_json;
    #[test]
    fn test_to_json_structure() {
        let json = to_json("48656c6c6f20576f726c64", "127.0.0.1", 7047);

        assert!(json.is_object());
        assert_eq!(json["presentationJson"]["version"], "0.1.0-alpha.8");
        assert_eq!(json["presentationJson"]["data"], "48656c6c6f20576f726c64");
        assert_eq!(json["presentationJson"]["meta"]["notaryUrl"], "https://127.0.0.1:7047");
        assert_eq!(json["presentationJson"]["meta"]["websocketProxyUrl"], "");
    }
}
