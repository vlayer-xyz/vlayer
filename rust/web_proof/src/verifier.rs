use k256::PublicKey;
use pkcs8::{EncodePublicKey, LineEnding};
use thiserror::Error;
use tlsn_core::signing::VerifyingKey;
use url::{ParseError, Url};

use crate::{
    errors::ParsingError,
    web::Web,
    web_proof::{VerificationError, WebProof},
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

    #[error("Public key conversion from sec1 format error: {0}")]
    ConversionFromSec1Format(#[from] k256::elliptic_curve::Error),

    #[error("Public key conversion to pem format error: {0}")]
    ConversionToPemFormat(#[from] pkcs8::spki::Error),
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, WebProofError> {
    let (request, response, server_name, notary_pub_key) = web_proof.verify()?;

    let web = Web {
        url: request.parse_url()?,
        server_name: server_name.to_string(),
        body: response.parse_body()?,
        notary_pub_key: to_pem_format(&notary_pub_key)?,
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

fn to_pem_format(verifying_key: &VerifyingKey) -> Result<String, WebProofError> {
    Ok(PublicKey::from_sec1_bytes(verifying_key.data.as_ref())?
        .to_public_key_pem(LineEnding::LF)?)
}

#[cfg(test)]
mod tests {
    use tlsn_core::signing::KeyAlgId;

    use super::*;
    use crate::{fixtures::load_web_proof_fixture, redaction::RedactionElementType};

    const X_TEST_URL: &str = "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true";

    mod verify_and_parse {
        use k256::PublicKey;
        use pkcs8::DecodePublicKey;
        use serde_json::Value;

        use super::*;
        use crate::fixtures::{read_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

        const WEB_PROOF_IDENTITY_NAME_CHANGED: &str =
            include_str!(".././testdata/web_proof_identity_name_changed.json");

        #[test]
        fn correct_url_extracted() {
            let web_proof = load_web_proof_fixture();

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.url, X_TEST_URL);
        }

        #[test]
        fn invalid_server_name() {
            let web_proof: WebProof =
                serde_json::from_str(WEB_PROOF_IDENTITY_NAME_CHANGED).unwrap();

            assert!(matches!(
                verify_and_parse(web_proof).err().unwrap(),
                WebProofError::Verification(VerificationError::Presentation(err)) if err.to_string() == "presentation error: server identity error caused by: server identity proof error: certificate: invalid server certificate"
            ));
        }

        #[test]
        fn correct_server_name_extracted() {
            let web_proof = load_web_proof_fixture();

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.server_name, "api.x.com");
        }

        #[test]
        fn correct_body_extracted() {
            let web_proof = load_web_proof_fixture();

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.body, "{\"protected\":false,\"screen_name\":\"g_p_vlayer\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"nsfw_user\":false,\"nsfw_admin\":false,\"ranked_timeline_setting\":null,\"ranked_timeline_eligible\":null,\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"alt_text_compose_enabled\":null,\"mention_filter\":\"unfiltered\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"ext_sharing_audiospaces_listening_data_with_followers\":true,\"ext\":{\"ssoConnections\":{\"r\":{\"ok\":[{\"ssoIdHash\":\"N2duh+nd63DR7ygWST+9ItxxOov5cwKQc21zK3NXVjY=\",\"ssoProvider\":\"Google\"}]},\"ttl\":-1}},\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{\"is_eu\":\"true\"}}");
        }

        #[test]
        fn correct_notary_pub_key() {
            let web_proof = load_web_proof_fixture();
            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(
                PublicKey::from_public_key_pem(&web.notary_pub_key).unwrap(),
                PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE).unwrap()
            );
        }

        #[test]
        #[ignore]
        fn success_all_redaction_turned_on() {
            let web_proof = read_fixture("./testdata/web_proof_all_redaction_types.json");
            let web_proof: WebProof = serde_json::from_str(&web_proof).unwrap();

            let web = verify_and_parse(web_proof).unwrap();

            let body = &web.body;
            let parsed: Value = serde_json::from_str(body).unwrap();

            let name = parsed.get("name").unwrap().as_str().unwrap();
            let eye_color = parsed.get("eye_color").unwrap().as_str().unwrap();

            assert_eq!(name, "Luke Skywalker");
            assert_eq!(eye_color, "****");

            let url = &web.url;
            assert_eq!(url, "https://swapi.dev/api/people/1?format=****");
        }

        #[test]
        #[ignore]
        fn fail_request_url_partial_redaction() {
            let web_proof = read_fixture("./testdata/web_proof_request_url_partial_redaction.json");
            let web_proof: WebProof = serde_json::from_str(&web_proof).unwrap();

            assert!(matches!(
                verify_and_parse(web_proof).err().unwrap(),
                WebProofError::Parsing(ParsingError::PartiallyRedactedValue(RedactionElementType::RequestUrlParam, err)) if err == "format: j***"
            ));
        }
        #[test]
        #[ignore]
        fn fail_request_header_partial_redaction() {
            let web_proof =
                read_fixture("./testdata/web_proof_request_header_partial_redaction.json");
            let web_proof: WebProof = serde_json::from_str(&web_proof).unwrap();

            assert!(matches!(
                verify_and_parse(web_proof).err().unwrap(),
                WebProofError::Parsing(ParsingError::PartiallyRedactedValue(RedactionElementType::RequestHeader, err)) if err == "connection: c****"
            ));
        }
        #[test]
        #[ignore]
        fn fail_response_header_partial_redaction() {
            let web_proof =
                read_fixture("./testdata/web_proof_response_header_partial_redaction.json");
            let web_proof: WebProof = serde_json::from_str(&web_proof).unwrap();

            assert!(matches!(
                verify_and_parse(web_proof).err().unwrap(),
                WebProofError::Parsing(ParsingError::PartiallyRedactedValue(RedactionElementType::ResponseHeader, err)) if err == "Server: n***********"
            ));
        }
        #[test]
        #[ignore]
        fn fail_response_body_json_value_partial_redaction() {
            let web_proof =
                read_fixture("./testdata/web_proof_response_json_partial_redaction.json");
            let web_proof: WebProof = serde_json::from_str(&web_proof).unwrap();

            assert!(matches!(
                verify_and_parse(web_proof).err().unwrap(),
                WebProofError::Parsing(ParsingError::PartiallyRedactedValue(RedactionElementType::ResponseBody, err)) if err == "$.eye_color: b***"
            ));
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

    mod to_pem_format {
        use pkcs8::DecodePublicKey;

        use super::*;
        use crate::fixtures::NOTARY_PUB_KEY_PEM_EXAMPLE;

        #[test]
        fn success() {
            let (_, _, _, verifying_key) = load_web_proof_fixture().verify().unwrap();

            let pem = to_pem_format(&verifying_key).unwrap();

            let expected_pem = PublicKey::from_public_key_pem(NOTARY_PUB_KEY_PEM_EXAMPLE)
                .unwrap()
                .to_public_key_pem(LineEnding::LF)
                .unwrap();

            assert_eq!(pem, expected_pem);
        }

        #[test]
        fn fail() {
            let verifying_key = VerifyingKey {
                alg: KeyAlgId::K256,
                data: vec![],
            };

            assert!(matches!(
                to_pem_format(&verifying_key).unwrap_err(),
                WebProofError::ConversionFromSec1Format(k256::elliptic_curve::Error)
            ));
        }
    }
}
