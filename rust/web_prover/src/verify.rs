use anyhow::{Result, anyhow};
use tlsn_core::{
    CryptoProvider,
    connection::ServerName,
    presentation::{Presentation, PresentationOutput},
    signing::VerifyingKey,
};

pub fn verify_presentation(presentation: Presentation) -> Result<VerificationResult> {
    let provider = CryptoProvider::default();

    let VerifyingKey { data: key_data, .. } = presentation.verifying_key();

    let encoded_key = hex::encode(key_data);

    let PresentationOutput {
        server_name,
        transcript,
        ..
    } = presentation.verify(&provider)?;

    let server_name = server_name.ok_or_else(|| anyhow!("server_name is missing"))?;
    let mut partial_transcript = transcript.ok_or_else(|| anyhow!("transcript is missing"))?;

    partial_transcript.set_unauthed(b'X');

    let sent = String::from_utf8_lossy(partial_transcript.sent_unsafe());
    let recv = String::from_utf8_lossy(partial_transcript.received_unsafe());

    Ok(VerificationResult {
        sent: sent.to_string(),
        recv: recv.to_string(),
        server_name,
        key: encoded_key,
    })
}

pub struct VerificationResult {
    pub sent: String,
    pub recv: String,
    pub server_name: ServerName,
    pub key: String,
}
