use std::string::FromUtf8Error;

use httparse::{Request, EMPTY_HEADER};
use thiserror::Error;
use tlsn_core::RedactedTranscript;

const MAX_HEADERS_NUMBER: usize = 40;

pub(crate) struct RequestTranscript {
    transcript: RedactedTranscript,
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
    fn test_parse_real_url() {
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
}
