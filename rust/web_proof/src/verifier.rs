use std::string::FromUtf8Error;

use http::header;
use tlsn_core::{
    proof::{SessionProofError, SubstringsProofError, TlsProof},
    RedactedTranscript, ServerName,
};

use crate::{
    types::WebProof,
    web_proof_parser::{parse_web_proof_request, ParserError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Session proof error: {0}")]
    SessionProof(#[from] SessionProofError),

    #[error("Substrings proof error: {0}")]
    SubstringsProof(#[from] SubstringsProofError),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Parse error: {0}")]
    Parser(#[from] ParserError),
}

pub struct Web {
    pub url: String,
    pub server_name: String,
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, VerificationError> {
    let ServerName::Dns(server_name) = web_proof.tls_proof.session.session_info.server_name.clone();
    let (sent, recv) = verify_proof(web_proof)?;
    let (sent_string, _recv_string) = extract_sent_recv_strings((sent, recv))?;
    let request_parse_result = parse_web_proof_request(&sent_string)?;
    let host_value = request_parse_result.header(header::HOST)?;

    Ok(Web {
        url: host_value.into(),
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
    sent.set_redacted(b'X');
    recv.set_redacted(b'X');

    let sent_string = String::from_utf8(sent.data().to_vec())?;
    let recv_string = String::from_utf8(recv.data().to_vec())?;

    Ok((sent_string, recv_string))
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
    fn correct_server_name_extracted() {
        let web_proof =
            load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);

        let web = verify_and_parse(web_proof).unwrap();

        assert_eq!(web.server_name, "api.x.com");
    }
}
