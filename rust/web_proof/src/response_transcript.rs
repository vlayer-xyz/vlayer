use httparse::{Response, EMPTY_HEADER};
use tlsn_core::RedactedTranscript;

use crate::request_transcript::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

pub(crate) struct ResponseTranscript {
    pub(crate) transcript: RedactedTranscript,
}

impl ResponseTranscript {
    pub fn new(transcript: RedactedTranscript) -> Self {
        Self { transcript }
    }

    pub(crate) fn parse_body(self) -> Result<String, ParsingError> {
        let response_string = String::from_utf8(self.transcript.data().to_vec())?;

        let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let mut res = Response::new(&mut headers);
        let body_index = res.parse(response_string.as_bytes())?.unwrap();

        let body = response_string[body_index..].to_string();

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::read_fixture;
    use tlsn_core::TranscriptSlice;

    #[test]
    fn success_parse_body() {
        let transcript = ResponseTranscript::new(RedactedTranscript::new(
            2690,
            vec![TranscriptSlice::new(
                0..2690,
                read_fixture("./testdata/received_response.txt")
                    .as_bytes()
                    .to_vec(),
            )],
        ));

        assert_eq!(transcript.parse_body().unwrap(), "{\"protected\":false,\"screen_name\":\"jab68503\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"nsfw_user\":false,\"nsfw_admin\":false,\"ranked_timeline_setting\":null,\"ranked_timeline_eligible\":null,\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"alt_text_compose_enabled\":null,\"mention_filter\":\"unfiltered\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"ext_sharing_audiospaces_listening_data_with_followers\":true,\"ext\":{\"ssoConnections\":{\"r\":{\"ok\":[{\"ssoIdHash\":\"P4GxOpBmKVdXcOWBZkVUlIJgrojh9RBwDDAEkGXK6VQ=\",\"ssoProvider\":\"Google\"}]},\"ttl\":-1}},\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{}}".to_string());
    }
}
