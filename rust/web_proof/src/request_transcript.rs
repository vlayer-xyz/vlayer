use derive_new::new;
use httparse::{Request, EMPTY_HEADER};

use crate::errors::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

#[derive(Debug, new)]
pub(crate) struct RequestTranscript {
    pub(crate) transcript: Vec<u8>,
}

impl RequestTranscript {
    pub(crate) fn parse_url(self) -> Result<String, ParsingError> {
        let request_string = String::from_utf8(self.transcript)?;

        let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let mut req = Request::new(&mut headers);
        req.parse(request_string.as_bytes())?;

        let url = req.path.ok_or(ParsingError::NoPathInRequest)?.to_string();
        Ok(url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::read_fixture;

    fn create_transcript(path: &str) -> RequestTranscript {
        RequestTranscript::new(read_fixture(path).as_bytes().to_vec())
    }

    #[test]
    fn parse_real_url_with_single_slice_transcript() {
        let transcript = create_transcript("./testdata/sent_request.txt");
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
        let transcript = RequestTranscript::new(vec![]);
        assert!(matches!(transcript.parse_url(), Err(ParsingError::NoPathInRequest)));
    }

    #[test]
    fn fail_not_utf8_transcript() {
        let transcript = RequestTranscript::new(vec![128]);
        assert!(matches!(
            transcript.parse_url(),
            Err(ParsingError::FromUtf8(err)) if err.to_string() == "invalid utf-8 sequence of 1 bytes from index 0"
        ));
    }
}
