use thiserror::Error;
use url::{ParseError, Url};

use crate::{
    errors::ParsingError,
    web::Web,
    web_proof::{VerificationError, WebProof, WebProofV7},
};

#[derive(Error, Debug)]
pub enum WebProofError {
    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),

    #[error("Request parsing error: {0}")]
    Parsing(#[from] ParsingError),

    #[error("Url parsing error: {0}")]
    ParseUrl(#[from] ParseError),

    #[error("No host found in the URL")]
    NoHostFoundInUrl,

    #[error("Host name extracted from url: {0} is different from server name: {1}")]
    HostNameMismatch(String, String),
}

pub fn verify_and_parse_v7(web_proof: WebProofV7) -> Result<Web, WebProofError> {
    verify_and_parse(web_proof.into())
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, WebProofError> {
    let notary_pub_key = web_proof
        .get_notary_pub_key()
        .map_err(VerificationError::PublicKeySerialization)?;
    let (request, response, server_name) = web_proof.verify()?;

    let web = Web {
        url: request.parse_url()?,
        server_name: server_name.to_string(),
        body: response.parse_body()?,
        notary_pub_key,
    };

    verify_server_name(server_name.as_str(), &web.url)?;

    Ok(web)
}

fn verify_server_name(server_name: &str, url: &str) -> Result<(), WebProofError> {
    let extracted_host = extract_host(url)?;
    if extracted_host == server_name {
        Ok(())
    } else {
        Err(WebProofError::HostNameMismatch(extracted_host, server_name.to_string()))
    }
}

fn extract_host(url: &str) -> Result<String, WebProofError> {
    Url::parse(url)?
        .host_str()
        .ok_or(WebProofError::NoHostFoundInUrl)
        .map(ToString::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::{load_web_proof_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    const X_TEST_URL: &str = "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true";

    const SWAPI_TEST_URL: &str = "https://swapi.dev/api/people/1";

    mod verify_and_parse_v7 {
        use p256::elliptic_curve::PublicKey;
        use pkcs8::DecodePublicKey;

        use super::*;
        use crate::{fixtures::read_fixture, web_proof::PresentationJson};

        #[test]
        fn correct_url_extracted() {
            let presentation_json = read_fixture("./testdata/presentation.json");
            let presentation_json: PresentationJson =
                serde_json::from_str(&presentation_json).unwrap();

            let web_proof = WebProofV7 {
                presentation_json,
                notary_pub_key: PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE).unwrap(),
            };

            let web = verify_and_parse_v7(web_proof).unwrap();

            assert_eq!(web.url, "https://api.x.com/1.1/account/settings.json");
        }
    }

    mod verify_and_parse {
        use super::*;

        #[test]
        fn correct_url_extracted() {
            let web_proof = load_web_proof_fixture(
                "./testdata/swapi_presentation_0.1.0-alpha.7.json",
                NOTARY_PUB_KEY_PEM_EXAMPLE,
            );

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.url, SWAPI_TEST_URL);
        }

        #[test]
        fn invalid_server_name() {
            // "wrong_server_name_tls_proof.json" is a real tls_proof, but with tampered server name, which the notary did not sign
            let web_proof = load_web_proof_fixture(
                "./testdata/swapi_presentation_0.1.0-alpha.7.invalid_cert.json",
                NOTARY_PUB_KEY_PEM_EXAMPLE,
            );

            assert!(matches!(
                verify_and_parse(web_proof).err().unwrap(),
                WebProofError::Verification(VerificationError::Presentation(err)) if err.to_string() == "presentation error: server identity error caused by: server identity proof error: commitment: certificate opening does not match commitment"
            ));
        }

        #[test]
        fn correct_server_name_extracted() {
            let web_proof = load_web_proof_fixture(
                "./testdata/swapi_presentation_0.1.0-alpha.7.json",
                NOTARY_PUB_KEY_PEM_EXAMPLE,
            );

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.server_name, "swapi.dev");
        }

        #[test]
        fn correct_body_extracted() {
            let web_proof = load_web_proof_fixture(
                "./testdata/swapi_presentation_0.1.0-alpha.7.json",
                NOTARY_PUB_KEY_PEM_EXAMPLE,
            );

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.body, "{\"name\":\"Luke Skywalker\",\"height\":\"172\",\"mass\":\"77\",\"hair_color\":\"blond\",\"skin_color\":\"fair\",\"eye_color\":\"blue\",\"birth_year\":\"19BBY\",\"gender\":\"male\",\"homeworld\":\"https://swapi.dev/api/planets/1/\",\"films\":[\"https://swapi.dev/api/films/1/\",\"https://swapi.dev/api/films/2/\",\"https://swapi.dev/api/films/3/\",\"https://swapi.dev/api/films/6/\"],\"species\":[],\"vehicles\":[\"https://swapi.dev/api/vehicles/14/\",\"https://swapi.dev/api/vehicles/30/\"],\"starships\":[\"https://swapi.dev/api/starships/12/\",\"https://swapi.dev/api/starships/22/\"],\"created\":\"2014-12-09T13:50:51.644000Z\",\"edited\":\"2014-12-20T21:17:56.891000Z\",\"url\":\"https://swapi.dev/api/people/1/\"}");
        }

        #[test]
        fn correct_notary_pub_key() {
            let web_proof = load_web_proof_fixture(
                "./testdata/swapi_presentation_0.1.0-alpha.7.json",
                NOTARY_PUB_KEY_PEM_EXAMPLE,
            );
            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.notary_pub_key, NOTARY_PUB_KEY_PEM_EXAMPLE);
        }
    }

    mod verify_server_name {
        use super::*;

        #[test]
        fn server_name_verification_success() {
            assert!(verify_server_name("api.x.com", X_TEST_URL).is_ok());
        }

        #[test]
        fn server_name_verification_fail_host_name_mismatch() {
            assert!(matches!(
                verify_server_name("x.com", X_TEST_URL).unwrap_err(),
                WebProofError::HostNameMismatch(host, server_name) if host == "api.x.com" && server_name == "x.com"
            ));
        }

        #[test]
        fn server_name_verification_fail_parse_url() {
            assert!(matches!(
                verify_server_name("", "").unwrap_err(),
                WebProofError::ParseUrl(ParseError::RelativeUrlWithoutBase)
            ));
        }

        #[test]
        fn server_name_verification_fail_host_not_found_in_url() {
            assert!(matches!(
                verify_server_name("", "unix:/a").unwrap_err(),
                WebProofError::NoHostFoundInUrl
            ));
        }
    }
}
