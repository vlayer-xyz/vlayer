use std::string::FromUtf8Error;

use httparse::{Request, EMPTY_HEADER};
use thiserror::Error;
use tlsn_core::RedactedTranscript;

const MAX_HEADERS_NUMBER: usize = 40;

#[derive(Debug)]
pub(crate) struct RequestTranscript {
    pub(crate) transcript: RedactedTranscript,
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("No path in request")]
    NoPathInRequest(),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Httparse error: {0}")]
    Httparse(#[from] httparse::Error),
}

impl RequestTranscript {
    pub fn new(transcript: RedactedTranscript) -> Self {
        Self { transcript }
    }

    pub(crate) fn parse_url(self) -> Result<String, ParsingError> {
        let request_string = String::from_utf8(self.transcript.data().to_vec())?;

        let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let mut req = Request::new(&mut headers);
        req.parse(request_string.as_bytes())?;

        let url = req.path.ok_or(ParsingError::NoPathInRequest())?.to_string();
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use tlsn_core::TranscriptSlice;

    use crate::fixtures::read_fixture;

    use super::*;

    #[test]
    fn parse_real_url_with_single_slice_transcript() {
        let transcript = RequestTranscript {
            transcript: RedactedTranscript::new(
                1998,
                vec![TranscriptSlice::new(
                    0..1998,
                    read_fixture("./testdata/sent_request.txt")
                        .as_bytes()
                        .to_vec(),
                )],
            ),
        };
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
        let transcript = RequestTranscript {
            transcript: RedactedTranscript::new(
                1735,
                vec![TranscriptSlice::new(
                    0..1735,
                    read_fixture("./testdata/redacted_sent_request.txt")
                        .as_bytes()
                        .to_vec(),
                )],
            ),
        };
        let url = transcript.parse_url();
        assert_eq!(
            url.unwrap_err().to_string(),
            "Httparse error: invalid header name"
        );
    }

    #[test]
    fn fail_to_many_headers() {
        let transcript = RequestTranscript {
            transcript: RedactedTranscript::new(
                5128,
                vec![TranscriptSlice::new(
                    0..5128,
                    read_fixture("./testdata/many_headers_sent_request.txt")
                        .as_bytes()
                        .to_vec(),
                )],
            ),
        };
        let url = transcript.parse_url();
        assert_eq!(
            url.unwrap_err().to_string(),
            "Httparse error: too many headers"
        );
    }
}
