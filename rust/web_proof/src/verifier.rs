use crate::{
    errors::ParsingError,
    web_proof::{VerificationError, WebProof},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WebProofError {
    #[error("Verification error: {0}")]
    Verification(#[from] VerificationError),

    #[error("Request parsing error: {0}")]
    Parsing(#[from] ParsingError),
}

pub struct Web {
    pub url: String,
    pub server_name: String,
    pub body: String,
}

pub fn verify_and_parse(web_proof: WebProof) -> Result<Web, WebProofError> {
    let server_name = web_proof.get_server_name();
    let (request, response) = web_proof.verify()?;

    Ok(Web {
        url: request.parse_url()?,
        server_name,
        body: response.parse_body()?,
    })
}

#[cfg(test)]
mod tests {
    use crate::fixtures::{load_web_proof_fixture, NOTARY_PUB_KEY_PEM_EXAMPLE};

    use super::*;

    #[test]
    fn correct_url_extracted() {
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

    #[test]
    fn correct_body_extracted() {
        let web_proof =
            load_web_proof_fixture("./testdata/tls_proof.json", NOTARY_PUB_KEY_PEM_EXAMPLE);

        let web = verify_and_parse(web_proof).unwrap();

        assert_eq!(web.body, "{\"protected\":false,\"screen_name\":\"jab68503\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"nsfw_user\":false,\"nsfw_admin\":false,\"ranked_timeline_setting\":null,\"ranked_timeline_eligible\":null,\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"alt_text_compose_enabled\":null,\"mention_filter\":\"unfiltered\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"ext_sharing_audiospaces_listening_data_with_followers\":true,\"ext\":{\"ssoConnections\":{\"r\":{\"ok\":[{\"ssoIdHash\":\"P4GxOpBmKVdXcOWBZkVUlIJgrojh9RBwDDAEkGXK6VQ=\",\"ssoProvider\":\"Google\"}]},\"ttl\":-1}},\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{}}");
    }
}
