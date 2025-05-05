use derive_new::new;

use crate::{
    errors::ParsingError, transcript_parser::parse_response_and_validate_redaction,
    web_proof::BodyRedactionMode,
};

#[derive(Debug, new)]
pub(crate) struct ResponseTranscript {
    pub(crate) transcript: Vec<u8>,
}

impl ResponseTranscript {
    pub(crate) fn parse_body(
        self,
        redaction_mode: BodyRedactionMode,
    ) -> Result<String, ParsingError> {
        parse_response_and_validate_redaction(&self.transcript, redaction_mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::read_fixture;

    const RESPONSE_BODY: &str = "{\"protected\":false,\"screen_name\":\"wktr0\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{\"is_eu\":\"true\"}}";

    #[test]
    fn parse_real_body_with_single_slice_transcript() {
        let transcript = ResponseTranscript::new(
            read_fixture("./testdata/received_response.txt")
                .as_bytes()
                .to_vec(),
        );

        assert_eq!(
            transcript.parse_body(BodyRedactionMode::Disabled).unwrap(),
            RESPONSE_BODY.to_string()
        );
    }

    const REDACTED_RESPONSE_BODY: &str = "{\"screen_name\":\"*****\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"nsfw_user\":false,\"nsfw_admin\":false,\"ranked_timeline_setting\":null,\"ranked_timeline_eligible\":null,\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"alt_text_compose_enabled\":null,\"mention_filter\":\"unfiltered\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"ext_sharing_audiospaces_listening_data_with_followers\":true,\"ext\":{\"ssoConnections\":{\"r\":{\"ok\":[{\"ssoIdHash\":\"AkGXHwarlY6pZFdEd3cbqfgdOyRufv9XiCdxLmfN884=\",\"ssoProvider\":\"Google\"}]},\"ttl\":-1}},\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{}}\r\n";

    #[test]
    fn redacted_body() {
        let transcript = ResponseTranscript::new(
            read_fixture("./testdata/redacted_received_response.txt")
                .as_bytes()
                .to_vec(),
        );

        let body = transcript.parse_body(BodyRedactionMode::EnabledUnsafe);
        assert_eq!(body.unwrap(), REDACTED_RESPONSE_BODY.to_string());
    }

    #[test]
    fn empty_response() {
        let transcript = ResponseTranscript::new(vec![]);
        assert!(matches!(
            transcript.parse_body(BodyRedactionMode::Disabled),
            Err(ParsingError::Partial)
        ));
    }

    #[test]
    fn no_headers_response() {
        let transcript = ResponseTranscript::new(
            read_fixture("./testdata/no_headers_response.txt")
                .as_bytes()
                .to_vec(),
        );

        assert!(matches!(
            transcript.parse_body(BodyRedactionMode::Disabled),
            Err(ParsingError::Httparse(httparse::Error::Version))
        ));
    }

    #[test]
    fn no_body_response() {
        let transcript = ResponseTranscript::new(
            read_fixture("./testdata/no_body_response.txt")
                .as_bytes()
                .to_vec(),
        );

        let err = transcript
            .parse_body(BodyRedactionMode::Disabled)
            .unwrap_err();
        assert!(
            matches!(err, ParsingError::Json(err) if err.to_string() == "EOF while parsing a value at line 1 column 0")
        );
    }

    #[test]
    fn error_not_utf8_transcript() {
        let transcript = ResponseTranscript::new(vec![128]);

        assert!(matches!(
            transcript.parse_body(BodyRedactionMode::Disabled),
            Err(ParsingError::Httparse(httparse::Error::Version))
        ));
    }

    #[test]
    fn parse_chunked_response_body() {
        let transcript = ResponseTranscript::new(
            read_fixture("./testdata/chunked_response.txt")
                .as_bytes()
                .to_vec(),
        );

        assert_eq!(
            transcript.parse_body(BodyRedactionMode::Disabled).unwrap(),
            "{\"name\":\"Luke Skywalker\",\"height\":\"172\",\"mass\":\"77\",\"hair_color\":\"blond\",\"skin_color\":\"fair\",\"eye_color\":\"blue\",\"birth_year\":\"19BBY\",\"gender\":\"male\",\"homeworld\":\"https://swapi.dev/api/planets/1/\",\"films\":[\"https://swapi.dev/api/films/1/\",\"https://swapi.dev/api/films/2/\",\"https://swapi.dev/api/films/3/\",\"https://swapi.dev/api/films/6/\"],\"species\":[],\"vehicles\":[\"https://swapi.dev/api/vehicles/14/\",\"https://swapi.dev/api/vehicles/30/\"],\"starships\":[\"https://swapi.dev/api/starships/12/\",\"https://swapi.dev/api/starships/22/\"],\"created\":\"2014-12-09T13:50:51.644000Z\",\"edited\":\"2014-12-20T21:17:56.891000Z\",\"url\":\"https://swapi.dev/api/people/1/\"}".to_string()
        );
    }
}
