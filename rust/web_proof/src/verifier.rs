use std::string::FromUtf8Error;

use httparse::{Header, Request, EMPTY_HEADER};
use tlsn_core::{
    proof::{SessionProofError, SubstringsProofError, TlsProof},
    RedactedTranscript, ServerName,
};

use crate::types::WebProof;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Session proof error: {0}")]
    SessionProof(#[from] SessionProofError),

    #[error("Substrings proof error: {0}")]
    SubstringsProof(#[from] SubstringsProofError),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Httparse error: {0}")]
    Httparse(#[from] httparse::Error),
}

pub struct Web {
    pub url: String,
    pub server_name: String,
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, VerificationError> {
    let ServerName::Dns(server_name) = web_proof.tls_proof.session.session_info.server_name.clone();
    let (sent, recv) = verify_proof(web_proof)?;
    let (sent_string, _recv_string) = extract_sent_recv_strings((sent, recv))?;

    let mut headers = [EMPTY_HEADER; 20];
    let mut req = Request::new(&mut headers);
    req.parse(sent_string.as_bytes())?;

    let host_value = find_value(req.headers, "host").expect("Host header not found");

    Ok(Web {
        url: host_value,
        server_name,
    })
}

fn verify_proof(
    web_proof: WebProof,
) -> Result<(RedactedTranscript, RedactedTranscript), VerificationError> {
    let TlsProof {
        session,
        substrings,
    } = web_proof.tls_proof;

    session.verify_with_default_cert_verifier(web_proof.notary_pub_key)?;

    Ok(substrings.verify(&session.header)?)
}

fn extract_sent_recv_strings(
    (mut sent, mut recv): (RedactedTranscript, RedactedTranscript),
) -> Result<(String, String), FromUtf8Error> {
    sent.set_redacted(0);
    recv.set_redacted(0);

    let mut sent_string = String::from_utf8(sent.data().to_vec())?.replace("\0\r\n", "");
    let mut recv_string = String::from_utf8(recv.data().to_vec())?.replace("\0\r\n", "");

    sent_string.retain(|c| c != '\0');
    recv_string.retain(|c| c != '\0');

    Ok((sent_string, recv_string))
}

fn find_value(headers: &[Header], name: &str) -> Option<String> {
    let header = headers.iter().find(|header| header.name == name);
    match header {
        Some(header) => Some(std::str::from_utf8(header.value).unwrap().to_string()),
        None => None,
    }
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
        assert!(verify_proof(invalid_proof).is_err());
    }

    #[test]
    fn success_verification() {
        let proof = load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);
        assert!(verify_proof(proof).is_ok());
    }

    #[test]
    fn correct_substrings_extracted() {
        let proof = load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);
        let (request, response) = extract_sent_recv_strings(verify_proof(proof).unwrap()).unwrap();

        assert_eq!(request, read_fixture("./testdata/sent_request.txt"));
        assert_eq!(response, read_fixture("./testdata/received_response.txt"));
    }

    #[test]
    fn correct_web_extracted() {
        let web_proof =
            load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);

        let web = verify_and_parse(web_proof).unwrap();

        assert_eq!(web.url, "api.x.com");
    }

    #[test]
    fn wrong_server_name() {
        // "wrong_server_name_tls_proof.json" is a real tls_proof, but with tampered server name, which the notary did not sign
        let web_proof = load_web_proof_fixture(
            "./testdata/wrong_server_name_tls_proof.json",
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        );

        assert!(verify_and_parse(web_proof).is_err());
    }

    #[test]
    fn correct_server_name_extracted() {
        let web_proof =
            load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);

        let web = verify_and_parse(web_proof).unwrap();

        assert_eq!(web.server_name, "api.x.com");
    }
}
