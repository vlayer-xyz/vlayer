use derive_new::new;
use httparse::{Request, EMPTY_HEADER};
use tlsn_core::RedactedTranscript;

use crate::errors::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

#[derive(Debug, new)]
pub(crate) struct RequestTranscript {
    pub(crate) transcript: RedactedTranscript,
}

impl RequestTranscript {
    pub(crate) fn parse_url(self) -> Result<String, ParsingError> {
        let request_string = String::from_utf8(self.transcript.data().to_vec())?;

        let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let mut req = Request::new(&mut headers);
        req.parse(request_string.as_bytes())?;

        let url = req.path.ok_or(ParsingError::NoPathInRequest)?.to_string();
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use tlsn_core::TranscriptSlice;

    use super::*;
    use crate::fixtures::read_fixture;

    fn create_transcript(path: &str) -> RequestTranscript {
        let transcript = read_fixture(path).as_bytes().to_vec();
        let transcript_length = transcript.len();
        RequestTranscript {
            transcript: RedactedTranscript::new(
                transcript_length,
                vec![TranscriptSlice::new(0..transcript_length, transcript)],
            ),
        }
    }

    #[test]
    fn parse_real_url_with_single_slice_transcript() {
        let transcript = create_transcript("./testdata/sent_request.txt");
        let url = transcript.parse_url().unwrap();
        assert_eq!(url, "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true");
    }

    #[test]
    fn parse_real_url_with_multiple_slice_transcript() {
        let transcript = RequestTranscript {
            transcript: RedactedTranscript::new(
                1998,
                vec![
                    TranscriptSlice::new(
                        0..1000,
                        read_fixture("./testdata/sent_request.txt").as_bytes()[0..1000].to_vec(),
                    ),
                    TranscriptSlice::new(
                        1000..1998,
                        read_fixture("./testdata/sent_request.txt").as_bytes()[1000..1998].to_vec(),
                    ),
                ],
            ),
        };
        let url = transcript.parse_url().unwrap();
        assert_eq!(url, "https://api.x.com/1.1/account/settings.json?include_ext_sharing_audiospaces_listening_data_with_followers=true&include_mention_filter=true&include_nsfw_user_flag=true&include_nsfw_admin_flag=true&include_ranked_timeline=true&include_alt_text_compose=true&ext=ssoConnections&include_country_code=true&include_ext_dm_nsfw_media_filter=true");
    }

    #[test]
    fn fail_redacted() {
        let transcript = create_transcript("./testdata/redacted_sent_request.txt");
        assert!(matches!(
            transcript.parse_url(),
            Err(ParsingError::Httparse(err)) if err.to_string() == "invalid header name"
        ));
    }

    #[test]
    fn fail_to_many_headers() {
        let transcript = create_transcript("./testdata/many_headers_sent_request.txt");
        assert!(matches!(
            transcript.parse_url(),
            Err(ParsingError::Httparse(err)) if err.to_string() == "too many headers"
        ));
    }

    #[test]
    fn fail_empty_transcript() {
        let transcript = RequestTranscript {
            transcript: RedactedTranscript::new(0, vec![]),
        };
        assert!(matches!(transcript.parse_url(), Err(ParsingError::NoPathInRequest)));
    }

    #[test]
    fn fail_not_utf8_transcript() {
        let transcript = RequestTranscript {
            transcript: RedactedTranscript::new(1, vec![TranscriptSlice::new(0..1, vec![128])]),
        };
        assert!(matches!(
            transcript.parse_url(),
            Err(ParsingError::FromUtf8(err)) if err.to_string() == "invalid utf-8 sequence of 1 bytes from index 0"
        ));
    }
}
