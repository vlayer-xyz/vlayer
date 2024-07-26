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

struct _WebProofRequestResponse {
    request: String,
    response: String,
}

fn _verify_proof(web_proof: WebProof) -> Result<_WebProofRequestResponse, VerificationError> {
    let TlsProof {
        session,
        substrings,
    } = web_proof.tls_proof;

    session.verify_with_default_cert_verifier(web_proof.notary_pub_key)?;

    let (mut sent, mut recv) = substrings.verify(&session.header)?;

    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    let sent_string = String::from_utf8(sent.data().to_vec()).unwrap();
    let recv_string = String::from_utf8(recv.data().to_vec()).unwrap();

    Ok(_WebProofRequestResponse {
        request: sent_string,
        response: recv_string,
    })
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{invalid_tls_proof_example, notary_pub_key_example, tls_proof_example};
    use crate::types::WebProof;

    use super::*;

    #[test]
    fn fail_verification() {
        let invalid_proof = WebProof {
            tls_proof: invalid_tls_proof_example(),
            notary_pub_key: notary_pub_key_example(),
        };
        assert!(_verify_proof(invalid_proof).is_err());
    }

    #[test]
    fn success_verification() {
        let proof = WebProof {
            tls_proof: tls_proof_example(),
            notary_pub_key: notary_pub_key_example(),
        };
        assert!(_verify_proof(proof).is_ok());
    }

    #[test]
    fn correct_substrings_extracted() {
        let proof = WebProof {
            tls_proof: tls_proof_example(),
            notary_pub_key: notary_pub_key_example(),
        };

        let result = _verify_proof(proof);
        assert!(result.is_ok());

        let _WebProofRequestResponse {
            request: sent,
            response: recv,
        } = result.unwrap();
        assert!(sent.contains("host: api.x.com"));
        assert!(recv.contains("HTTP/1.1 200 OK"));
    }
}
