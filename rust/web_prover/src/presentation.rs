use tlsn_core::{attestation::Attestation, presentation::Presentation, CryptoProvider, Secrets};
use tracing::debug;
use utils::range::RangeSet;

pub async fn create_presentation(
    attestation: Attestation,
    secrets: Secrets,
) -> Result<Presentation, Box<dyn std::error::Error>> {
    debug!("Creating presentation");

    let mut builder = secrets.transcript_proof_builder();

    let transcript = secrets.transcript();

    builder.reveal_sent(&RangeSet::from(0..transcript.sent().len()))?;
    builder.reveal_recv(&RangeSet::from(0..transcript.received().len()))?;

    let transcript_proof = builder.build()?;

    let provider = CryptoProvider::default();
    let mut builder = attestation.presentation_builder(&provider);

    builder
        .identity_proof(secrets.identity_proof())
        .transcript_proof(transcript_proof);

    Ok(builder.build()?)
}
