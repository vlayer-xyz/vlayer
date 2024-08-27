use std::{str::Utf8Error, string::FromUtf8Error};

use tlsn_core::{
    proof::{SessionProofError, SubstringsProofError, TlsProof},
    RedactedTranscript, ServerName,
};

use crate::{
    request_transcript::{ParsingError, RequestTranscript},
    types::WebProof,
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

    #[error("utf8 error: {0}")]
    Utf8(#[from] Utf8Error),

    #[error("Httparse error: {0}")]
    Httparse(#[from] httparse::Error),

    #[error("No header found: {0}")]
    NoHeaderFound(String),

    #[error("Request parsing error: {0}")]
    ParsingError(#[from] ParsingError),
}

pub struct Web {
    pub url: String,
    pub server_name: String,
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, VerificationError> {
    let ServerName::Dns(server_name) = web_proof.tls_proof.session.session_info.server_name.clone();
    let (sent, _recv) = verify_proof(web_proof)?;
    let request = RequestTranscript::new(sent);

    let url = request.parse_url()?;

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

#[cfg(test)]
mod tests {
    use crate::fixtures::{load_web_proof_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

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
