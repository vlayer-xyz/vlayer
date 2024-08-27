use tlsn_core::{proof::TlsProof, RedactedTranscript, ServerName};

use crate::{
    request_transcript::{ParsingError, RequestTranscript},
    web_proof::{VerificationError, WebProof},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebCreationError {
    #[error("Verification error: {0}")]
    VerificationError(#[from] VerificationError),

    #[error("Request parsing error: {0}")]
    ParsingError(#[from] ParsingError),
}

pub struct Web {
    pub url: String,
    pub server_name: String,
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, WebCreationError> {
    let request = web_proof.verify()?;

    Ok(Web {
        url: request.parse_url()?,
        server_name: web_proof.get_server_name(),
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

#[cfg(test)]
mod tests {
    use crate::fixtures::{load_web_proof_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    use super::*;

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
