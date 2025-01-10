use std::iter::zip;

use derive_new::new;
use httparse::{Header, Request, EMPTY_HEADER};

use crate::errors::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

const REDACTED_CHAR: char = '\0';

// Both '-' and '+' are valid header characters. Replacing redacted '\0' bytes with them
// ensures the request is parsable and allows analysis of redacted content via diffs.
const HEADER_NAME_REPLACEMENT_CHAR_1: char = '-';
const HEADER_NAME_REPLACEMENT_CHAR_2: char = '+';

#[derive(Debug, new)]
pub(crate) struct RequestTranscript {
    pub(crate) transcript: Vec<u8>,
}

impl RequestTranscript {
    pub(crate) fn parse_url(self) -> Result<String, ParsingError> {
        let request = String::from_utf8(self.transcript)?;

        extract_path_and_check_proper_headers_redaction(&request)
    }
}

fn extract_path_and_check_proper_headers_redaction(request: &str) -> Result<String, ParsingError> {
    let request_string =
        request.replace(REDACTED_CHAR, HEADER_NAME_REPLACEMENT_CHAR_1.to_string().as_str());
    let (path, headers_with_replacement_1) = parse_request(&request_string)?;

    let request_string =
        request.replace(REDACTED_CHAR, HEADER_NAME_REPLACEMENT_CHAR_2.to_string().as_str());
    let (_, headers_with_replacement_2) = parse_request(&request_string)?;

    validate_headers_redaction(&headers_with_replacement_1, &headers_with_replacement_2)?;

    Ok(path)
}

fn parse_request(request: &str) -> Result<(String, [Header; MAX_HEADERS_NUMBER]), ParsingError> {
    let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
    let mut req = Request::new(&mut headers);
    req.parse(request.as_bytes())?;

    let path = req.path.ok_or(ParsingError::NoPathInRequest)?.to_string();

    Ok((path, headers))
}

fn validate_headers_redaction(
    headers_with_replacement_1: &[Header; MAX_HEADERS_NUMBER],
    headers_with_replacement_2: &[Header; MAX_HEADERS_NUMBER],
) -> Result<(), ParsingError> {
    let header_pairs = zip(headers_with_replacement_1.iter(), headers_with_replacement_2.iter());

    let some_header_name_is_redacted = header_pairs.clone().any(|(l, r)| l.name != r.name);

    if some_header_name_is_redacted {
        return Err(ParsingError::RedactedHeaderName);
    }

    let some_header_value_is_partially_redacted = header_pairs.clone().any(|(l, r)| {
        !fully_redacted(l.value, HEADER_NAME_REPLACEMENT_CHAR_1) && l.value != r.value
    });

    if some_header_value_is_partially_redacted {
        return Err(ParsingError::PartiallyRedactedHeaderValue);
    }

    Ok(())
}

fn fully_redacted(input: &[u8], redacted_char: char) -> bool {
    input.iter().all(|&c| c == redacted_char as u8)
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
        assert_eq!(url, "https://api.x.com/1.1/account/settings.json");
    }

    mod header_redaction {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn no_header_redaction() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json\r\n\r\n";
                let url = extract_path_and_check_proper_headers_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_name_with_replacement_character_1() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{HEADER_NAME_REPLACEMENT_CHAR_1}: application/json\r\n\r\n");
                let url =
                    extract_path_and_check_proper_headers_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_name_with_replacement_character_2() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{HEADER_NAME_REPLACEMENT_CHAR_2}: application/json\r\n\r\n");
                let url =
                    extract_path_and_check_proper_headers_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_value_with_replacement_character_1() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{HEADER_NAME_REPLACEMENT_CHAR_1}\r\n\r\n");
                let url =
                    extract_path_and_check_proper_headers_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_value_with_replacement_character_2() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{HEADER_NAME_REPLACEMENT_CHAR_2}\r\n\r\n");
                let url =
                    extract_path_and_check_proper_headers_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn fully_redacted_request_header_value() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                let url = extract_path_and_check_proper_headers_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn fully_redacted_request_header_value_no_space_before_value() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                let url = extract_path_and_check_proper_headers_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }
        }

        mod fail {
            use super::*;

            #[test]
            fn partially_redacted_request_header_value() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/jso\0\r\n\r\n";
                let err = extract_path_and_check_proper_headers_redaction(request).unwrap_err();
                assert!(matches!(err, ParsingError::PartiallyRedactedHeaderValue));
            }

            #[test]
            fn partially_redacted_request_header_name() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-typ\0: application/json\r\n\r\n";
                let err = extract_path_and_check_proper_headers_redaction(request).unwrap_err();
                assert!(matches!(err, ParsingError::RedactedHeaderName));
            }

            #[test]
            fn fully_redacted_request_header_name() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0: application/json\r\n\r\n";
                let err = extract_path_and_check_proper_headers_redaction(request).unwrap_err();
                assert!(matches!(err, ParsingError::RedactedHeaderName));
            }

            #[test]
            fn fully_redacted_request_header_name_and_value() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                let err = extract_path_and_check_proper_headers_redaction(request).unwrap_err();
                assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
            }
        }
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
