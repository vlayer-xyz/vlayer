use thiserror::Error;
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
    use crate::fixtures::load_web_proof_fixture;

    const X_TEST_URL: &str = "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true";

    mod verify_and_parse {
        use k256::PublicKey;
        use pkcs8::DecodePublicKey;

        use super::*;
        use crate::fixtures::{
            utils::{change_server_name, load_web_proof_fixture_and_modify},
            NOTARY_PUB_KEY_PEM_EXAMPLE,
        };

        #[test]
        fn correct_url_extracted() {
            let web_proof = load_web_proof_fixture();

            let web = verify_and_parse(web_proof).unwrap();

            assert_eq!(web.url, "https://api.x.com/1.1/account/settings.json");
        }

        #[test]
        fn invalid_server_name() {
            let web_proof = load_web_proof_fixture_and_modify(change_server_name);

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

            assert_eq!(web.body, "{\"protected\":false,\"screen_name\":\"wktr0\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{\"is_eu\":\"true\"}}");
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
