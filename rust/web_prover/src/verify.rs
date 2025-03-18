use std::error::Error;

use tlsn_core::{
    connection::ServerName,
    presentation::{Presentation, PresentationOutput},
    signing::VerifyingKey,
    CryptoProvider,
};

pub fn verify_presentation(
    presentation: Presentation,
) -> Result<VerificationResult, Box<dyn Error>> {
    let provider = CryptoProvider::default();

    let VerifyingKey { data: key_data, .. } = presentation.verifying_key();

    let encoded_key = hex::encode(key_data);

    // Verify the presentation.
    let PresentationOutput {
        server_name,
        transcript,
        ..
    } = presentation.verify(&provider).unwrap();

    let server_name = server_name.unwrap();
    let mut partial_transcript = transcript.unwrap();
    // Set the unauthenticated bytes so they are distinguishable.
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
