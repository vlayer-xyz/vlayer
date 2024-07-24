use tlsn_core::proof::{SessionProofError, SubstringsProofError, TlsProof};

use crate::types::WebProof;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Session proof error: {0}")]
    SessionProof(#[from] SessionProofError),

    #[error("Substrings proof error: {0}")]
    SubstringsProof(#[from] SubstringsProofError),
}

fn _verify_proof(web_proof: WebProof) -> Result<(), VerificationError> {
    let TlsProof {
        session,
        substrings,
    } = web_proof.tls_proof;

    session.verify_with_default_cert_verifier(web_proof.notary_pub_key)?;

    substrings.verify(&session.header)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use tlsn_core::proof::{SessionProof, TlsProof};

    use crate::fixtures::{notary_pub_key_example, tls_proof_example};
    use crate::types::WebProof;

    use super::*;

    #[test]
    fn fail_verification() {
        let tls_proof = tls_proof_example();

        let wrong_tls_proof = TlsProof {
            session: SessionProof {
                signature: None,
                ..tls_proof.session
            },
            ..tls_proof
        };

        let proof = WebProof {
            tls_proof: wrong_tls_proof,
            notary_pub_key: notary_pub_key_example(),
        };
        assert!(_verify_proof(proof).is_err());
    }

    #[test]
    fn success_verification() {
        let proof = WebProof {
            tls_proof: tls_proof_example(),
            notary_pub_key: notary_pub_key_example(),
        };
        assert!(_verify_proof(proof).is_ok());
    }
}
