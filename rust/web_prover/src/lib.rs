mod notarize;
mod presentation;
mod verify;

pub use notarize::notarize;
pub use presentation::create_presentation;
pub use verify::verify_presentation;

pub async fn generate_web_proof(
    notary_host: &str,
    notary_port: u16,
    server_domain: &str,
    server_host: &str,
    server_port: u16,
    uri: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (attestation, secrets) =
        notarize(notary_host, notary_port, server_domain, server_host, server_port, uri).await?;
    let presentation = create_presentation(attestation, secrets).await?;

    let json_response = serde_json::json!({
        "data": hex::encode(bincode::serialize(&presentation)?)
    });

    Ok(serde_json::to_string(&json_response)?)
}
