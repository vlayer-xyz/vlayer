use tlsn_core::{attestation::Attestation, presentation::Presentation, CryptoProvider, Secrets};
use tracing::debug;

use crate::RedactionConfig;

pub fn create_presentation_with_redaction(
    attestation: &Attestation,
    secrets: &Secrets,
    redaction_config: &RedactionConfig,
) -> Result<Presentation, Box<dyn std::error::Error>> {
    debug!("Creating presentation");

    let mut builder = secrets.transcript_proof_builder();

    builder.reveal_sent(&redaction_config.sent)?;
    builder.reveal_recv(&redaction_config.recv)?;

    let transcript_proof = builder.build()?;

    let provider = CryptoProvider::default();
    let mut builder = attestation.presentation_builder(&provider);

    builder
        .identity_proof(secrets.identity_proof())
        .transcript_proof(transcript_proof);

    Ok(builder.build()?)
}
