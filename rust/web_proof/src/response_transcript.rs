use httparse::{Response, Status, EMPTY_HEADER};
use tlsn_core::RedactedTranscript;

use crate::errors::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

#[derive(Debug)]
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
        let body_index = match res.parse(response_string.as_bytes())? {
            Status::Complete(t) => t,
            Status::Partial => return Err(ParsingError::Partial),
        };

        let body = response_string[body_index..].to_string();

        Ok(body)
    }
}

#[cfg(test)]
mod tests {
    use tlsn_core::TranscriptSlice;

    use super::*;
    use crate::fixtures::read_fixture;

    const RESPONSE_BODY: &str = "{\"protected\":false,\"screen_name\":\"jab68503\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"nsfw_user\":false,\"nsfw_admin\":false,\"ranked_timeline_setting\":null,\"ranked_timeline_eligible\":null,\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"alt_text_compose_enabled\":null,\"mention_filter\":\"unfiltered\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"ext_sharing_audiospaces_listening_data_with_followers\":true,\"ext\":{\"ssoConnections\":{\"r\":{\"ok\":[{\"ssoIdHash\":\"P4GxOpBmKVdXcOWBZkVUlIJgrojh9RBwDDAEkGXK6VQ=\",\"ssoProvider\":\"Google\"}]},\"ttl\":-1}},\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{}}";

    #[test]
    fn parse_real_body_with_single_slice_transcript() {
        let transcript = ResponseTranscript::new(RedactedTranscript::new(
            2690,
            vec![TranscriptSlice::new(
                0..2690,
                read_fixture("./testdata/received_response.txt")
                    .as_bytes()
                    .to_vec(),
            )],
        ));

        assert_eq!(transcript.parse_body().unwrap(), RESPONSE_BODY.to_string());
    }

    #[test]
    fn parse_real_body_with_multiple_slice_transcript() {
        let transcript = ResponseTranscript::new(RedactedTranscript::new(
            2690,
            vec![
                TranscriptSlice::new(
                    0..2000,
                    read_fixture("./testdata/received_response.txt").as_bytes()[0..2000].to_vec(),
                ),
                TranscriptSlice::new(
                    2000..2690,
                    read_fixture("./testdata/received_response.txt").as_bytes()[2000..2690]
                        .to_vec(),
                ),
            ],
        ));

        assert_eq!(transcript.parse_body().unwrap(), RESPONSE_BODY.to_string());
    }

    const REDACTED_RESPONSE_BODY: &str = "XXXXXXXXXXXXXXXXXXX\"screen_name\":\"wktr0\",\"always_use_https\":true,\"use_cookie_personalization\":false,\"sleep_time\":{\"enabled\":false,\"end_time\":null,\"start_time\":null},\"geo_enabled\":false,\"language\":\"en\",\"discoverable_by_email\":false,\"discoverable_by_mobile_phone\":false,\"display_sensitive_media\":false,\"personalized_trends\":true,\"allow_media_tagging\":\"all\",\"allow_contributor_request\":\"none\",\"allow_ads_personalization\":false,\"allow_logged_out_device_personalization\":false,\"allow_location_history_personalization\":false,\"allow_sharing_data_for_third_party_personalization\":false,\"allow_dms_from\":\"following\",\"always_allow_dms_from_subscribers\":null,\"allow_dm_groups_from\":\"following\",\"translator_type\":\"none\",\"country_code\":\"pl\",\"nsfw_user\":false,\"nsfw_admin\":false,\"ranked_timeline_setting\":null,\"ranked_timeline_eligible\":null,\"address_book_live_sync_enabled\":false,\"universal_quality_filtering_enabled\":\"enabled\",\"dm_receipt_setting\":\"all_enabled\",\"alt_text_compose_enabled\":null,\"mention_filter\":\"unfiltered\",\"allow_authenticated_periscope_requests\":true,\"protect_password_reset\":false,\"require_password_login\":false,\"requires_login_verification\":false,\"ext_sharing_audiospaces_listening_data_with_followers\":true,\"ext\":{\"ssoConnections\":{\"r\":{\"ok\":[{\"ssoIdHash\":\"AkGXHwarlY6pZFdEd3cbqfgdOyRufv9XiCdxLmfN884=\",\"ssoProvider\":\"Google\"}]},\"ttl\":-1}},\"dm_quality_filter\":\"enabled\",\"autoplay_disabled\":false,\"settings_metadata\":{}}";

    #[test]
    fn redacted_body() {
        let transcript = ResponseTranscript {
            transcript: RedactedTranscript::new(
                2687,
                vec![TranscriptSlice::new(
                    0..2687,
                    read_fixture("./testdata/redacted_received_response.txt")
                        .as_bytes()
                        .to_vec(),
                )],
            ),
        };

        let body = transcript.parse_body();
        assert_eq!(body.unwrap(), REDACTED_RESPONSE_BODY.to_string());
    }

    #[test]
    fn empty_response() {
        let transcript = ResponseTranscript {
            transcript: RedactedTranscript::new(0, vec![TranscriptSlice::new(0..0, vec![])]),
        };

        assert!(matches!(transcript.parse_body(), Err(ParsingError::Partial)));
    }

    #[test]
    fn no_headers_response() {
        let transcript = ResponseTranscript {
            transcript: RedactedTranscript::new(
                1432,
                vec![TranscriptSlice::new(
                    0..1432,
                    read_fixture("./testdata/no_headers_response.txt")
                        .as_bytes()
                        .to_vec(),
                )],
            ),
        };

        assert!(matches!(
            transcript.parse_body(),
            Err(ParsingError::Httparse(httparse::Error::Version))
        ));
    }

    #[test]
    fn no_body_response() {
        let transcript = ResponseTranscript {
            transcript: RedactedTranscript::new(
                1258,
                vec![TranscriptSlice::new(
                    0..1258,
                    read_fixture("./testdata/no_body_response.txt")
                        .as_bytes()
                        .to_vec(),
                )],
            ),
        };

        let body = transcript.parse_body();
        assert_eq!(body.unwrap(), "".to_string());
    }

    #[test]
    fn error_not_utf8_transcript() {
        let transcript = ResponseTranscript {
            transcript: RedactedTranscript::new(1, vec![TranscriptSlice::new(0..1, vec![128])]),
        };

        assert!(matches!(
            transcript.parse_body(),
            Err(ParsingError::FromUtf8(err)) if err.to_string() == "invalid utf-8 sequence of 1 bytes from index 0"
        ));
    }
}
