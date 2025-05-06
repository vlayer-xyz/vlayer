use derive_new::new;

use crate::{
    errors::ParsingError, transcript_parser::parse_request_and_validate_redaction,
    web_proof::UrlTestMode,
};

#[derive(Debug, new)]
pub(crate) struct RequestTranscript {
    pub(crate) transcript: Vec<u8>,
}

impl RequestTranscript {
    pub(crate) fn parse_url(self, url_test_mode: UrlTestMode) -> Result<String, ParsingError> {
        parse_request_and_validate_redaction(&self.transcript, url_test_mode)
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
        let url = transcript.parse_url(UrlTestMode::Full).unwrap();
        assert_eq!(url, "https://api.x.com/1.1/account/settings.json");
    }

    #[test]
    fn fail_redacted() {
        let transcript = create_transcript("./testdata/redacted_sent_request.txt");
        assert!(matches!(
            transcript.parse_url(UrlTestMode::Full),
            Err(ParsingError::Httparse(err)) if err.to_string() == "invalid header name"
        ));
    }

    #[test]
    fn fail_to_many_headers() {
        let transcript = create_transcript("./testdata/many_headers_sent_request.txt");
        assert!(matches!(
            transcript.parse_url(UrlTestMode::Full),
            Err(ParsingError::Httparse(err)) if err.to_string() == "too many headers"
        ));
    }

    #[test]
    fn fail_empty_transcript() {
        let transcript = RequestTranscript::new(vec![]);
        assert!(matches!(
            transcript.parse_url(UrlTestMode::Full),
            Err(ParsingError::NoHttpMethodInRequest)
        ));
    }

    #[test]
    fn fail_not_utf8_transcript() {
        let transcript = RequestTranscript::new(vec![128]);
        assert!(matches!(
            transcript.parse_url(UrlTestMode::Full),
            Err(ParsingError::Httparse(httparse::Error::Token))
        ));
    }
}
