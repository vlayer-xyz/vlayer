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

struct _WebProofJournal {
    request: String,
    response: String,
}

fn _verify_proof(web_proof: WebProof) -> Result<_WebProofJournal, VerificationError> {
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

    Ok(_WebProofJournal {
        request: sent_string,
        response: recv_string,
    })
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{load_web_proof_fixture, read_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    use super::*;

    #[test]
    fn fail_verification() {
        let invalid_proof = load_web_proof_fixture(
            "./testdata/invalid_tls_proof.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );
        assert!(_verify_proof(invalid_proof).is_err());
    }

    #[test]
    fn success_verification() {
        let proof = load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);
        assert!(_verify_proof(proof).is_ok());
    }

    #[test]
    fn correct_substrings_extracted() {
        let proof = load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);
        let _WebProofJournal { request, response } = _verify_proof(proof).unwrap();

        assert_eq!(request, read_fixture("./testdata/sent_request.txt"));
        assert_eq!(response, read_fixture("./testdata/received_response.txt"));
    }
}
