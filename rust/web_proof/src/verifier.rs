use std::{str::Utf8Error, string::FromUtf8Error};

use httparse::{Header, Request, Response, EMPTY_HEADER};
use tlsn_core::{
    proof::{SessionProofError, SubstringsProofError, TlsProof},
    RedactedTranscript, ServerName,
};

use crate::types::WebProof;
use thiserror::Error;

const MAX_HEADERS_NUMBER: usize = 40;

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Session proof error: {0}")]
    SessionProof(#[from] SessionProofError),

    #[error("Substrings proof error: {0}")]
    SubstringsProof(#[from] SubstringsProofError),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("utf8 error: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("Httparse error: {0}")]
    Httparse(#[from] httparse::Error),

    #[error("No header found: {0}")]
    NoHeaderFound(String),
}

pub struct Web {
    pub url: String,
    pub server_name: String,
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, VerificationError> {
    let ServerName::Dns(server_name) = web_proof.tls_proof.session.session_info.server_name.clone();
    let (sent, recv) = verify_proof(web_proof)?;
    let (sent_string, _recv_string) = extract_sent_recv_strings((sent, recv))?;

    let mut req_headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
    let req = parse_tlsn_http_request(&sent_string, &mut req_headers)?;

    let url = req
        .path
        .ok_or(VerificationError::NoHeaderFound("path".to_string()))?
        .to_string();

    Ok(Web { url, server_name })
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

fn parse_tlsn_http_request<'a>(
    tlsn_http_request: &'a str,
    headers: &'a mut [Header<'a>],
) -> Result<Request<'a, 'a>, VerificationError> {
    let mut req = Request::new(headers);
    req.parse(tlsn_http_request.as_bytes())?;
    Ok(req)
}

fn _parse_tlsn_http_response<'a>(
    tlsn_http_response: &'a str,
    headers: &'a mut [Header<'a>],
) -> Result<Response<'a, 'a>, VerificationError> {
    let mut res = Response::new(headers);
    res.parse(tlsn_http_response.as_bytes())?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{load_web_proof_fixture, read_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    use super::*;

    #[test]
    fn fail_too_many_headers() {
        let request = read_fixture("./testdata/sent_request.txt");
        let mut headers = [EMPTY_HEADER; 1];
        let req = parse_tlsn_http_request(&request, &mut headers);
        assert_eq!(
            req.unwrap_err().to_string(),
            "Httparse error: too many headers"
        );
    }

    #[test]
    fn correct_parsing_request() {
        let request = read_fixture("./testdata/sent_request.txt");
        let mut req_headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let req = parse_tlsn_http_request(&request, &mut req_headers).unwrap();

        assert_eq!(req.method.unwrap(), "GET");
        assert_eq!(req.path.unwrap(), "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true");
        assert_eq!(req.version.unwrap(), 1);
        assert_eq!(req.headers.len(), 21);
        assert_eq!(req.headers[5].name, "host");
        assert_eq!(req.headers[5].value, "api.x.com".as_bytes());
    }

    #[test]
    fn correct_parsing_response() {
        let response = read_fixture("./testdata/received_response.txt");
        let mut res_headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let res = _parse_tlsn_http_response(&response, &mut res_headers).unwrap();

        assert_eq!(res.version.unwrap(), 1);
        assert_eq!(res.code.unwrap(), 200);
        assert_eq!(res.reason.unwrap(), "OK");
        assert_eq!(res.headers.len(), 26);
        assert_eq!(res.headers[4].name, "status");
        assert_eq!(res.headers[4].value, "200 OK".as_bytes());
    }

    #[test]
    fn fail_redacted() {
        let redacted_request = read_fixture("./testdata/redacted_sent_request.txt");
        let mut req_headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let req = parse_tlsn_http_request(&redacted_request, &mut req_headers);
        assert_eq!(
            req.unwrap_err().to_string(),
            "Httparse error: invalid header name"
        );
    }

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

        assert_eq!(web.url, "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true");
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
