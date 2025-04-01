use tlsn_core::{attestation::Attestation, presentation::Presentation, CryptoProvider, Secrets};
use tlsn_formats::http::HttpTranscript;
use tracing::debug;

pub async fn create_presentation(
    attestation: Attestation,
    secrets: Secrets,
) -> Result<Presentation, Box<dyn std::error::Error>> {
    debug!("Creating presentation");
    // let transcript = HttpTranscript::parse(secrets.transcript())?;

    let mut builder = secrets.transcript_proof_builder();

    // let request = &transcript.requests[0];
    // builder.reveal_sent(request)?;

    // let response = &transcript.responses[0];
    // builder.reveal_recv(response)?;

    let transcript_proof = builder.build()?;

    let provider = CryptoProvider::default();
    let mut builder = attestation.presentation_builder(&provider);

    builder
        .identity_proof(secrets.identity_proof())
        .transcript_proof(transcript_proof);

    Ok(builder.build()?)
}
