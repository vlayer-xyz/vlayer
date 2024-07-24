use tlsn_core::proof::TlsProof;

use crate::types::WebProof;

#[derive(PartialEq, Debug)]
struct VerificationError;

fn verify_proof(web_proof: WebProof) -> Result<(), VerificationError> {
    let TlsProof {
        session,
        substrings,
    } = web_proof.tls_proof;

    session
        .verify_with_default_cert_verifier(web_proof.notary_pub_key)
        .map_err(|_err| VerificationError)?;

    substrings
        .verify(&session.header)
        .map_err(|_err| VerificationError)?;

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
        assert_eq!(verify_proof(proof), Err(VerificationError))
    }

    #[test]
    fn success_verification() {
        let proof = WebProof {
            tls_proof: tls_proof_example(),
            notary_pub_key: notary_pub_key_example(),
        };
        assert_eq!(verify_proof(proof), Ok(()))
    }
}
