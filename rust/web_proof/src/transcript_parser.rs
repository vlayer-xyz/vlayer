use std::{convert::Into, iter::zip};

use httparse::{Header, Request, Response, Status, EMPTY_HEADER};

use crate::errors::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

const REDACTED_CHAR: char = '\0';

// Both '-' and '+' are valid header characters. Replacing redacted '\0' bytes with
// two different characters ensures the request is parsable and allows analysis
// of redacted content via diffs.
const HEADER_NAME_REPLACEMENT_CHAR_1: char = '-';
const HEADER_NAME_REPLACEMENT_CHAR_2: char = '+';

#[derive(Clone, Debug)]
pub(crate) struct HttpHeader {
    pub(crate) name: String,
    pub(crate) value: Vec<u8>,
}

impl From<Header<'_>> for HttpHeader {
    fn from(header: Header) -> Self {
        HttpHeader {
            name: header.name.to_string(),
            value: header.value.to_vec(),
        }
    }
}

pub(crate) fn parse_request_and_validate_redaction(request: &str) -> Result<String, ParsingError> {
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

pub(crate) fn parse_response_and_validate_redaction(
    response: &str,
) -> Result<(String, Vec<HttpHeader>), ParsingError> {
    let response_string =
        response.replace(REDACTED_CHAR, HEADER_NAME_REPLACEMENT_CHAR_1.to_string().as_str());
    let (body_index, headers_with_replacement_1) = parse_response(&response_string)?;

    let response_string =
        response.replace(REDACTED_CHAR, HEADER_NAME_REPLACEMENT_CHAR_2.to_string().as_str());
    let (_, headers_with_replacement_2) = parse_response(&response_string)?;

    validate_headers_redaction(&headers_with_replacement_1, &headers_with_replacement_2)?;

    let body = &response_string[body_index..];

    Ok((body.to_string(), headers_with_replacement_1.map(Into::into).to_vec()))
}

fn parse_response(response: &str) -> Result<(usize, [Header; MAX_HEADERS_NUMBER]), ParsingError> {
    let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
    let mut res = Response::new(&mut headers);
    let body_index = match res.parse(response.as_bytes())? {
        Status::Complete(t) => t,
        Status::Partial => return Err(ParsingError::Partial),
    };

    Ok((body_index, headers))
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

    mod header_redaction {
        use super::*;

        mod success {
            use super::*;

            #[test]
            fn request_no_header_redaction() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json\r\n\r\n";
                let url = parse_request_and_validate_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_name_with_replacement_character_1() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{HEADER_NAME_REPLACEMENT_CHAR_1}: application/json\r\n\r\n");
                let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_name_with_replacement_character_2() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{HEADER_NAME_REPLACEMENT_CHAR_2}: application/json\r\n\r\n");
                let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_value_with_replacement_character_1() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{HEADER_NAME_REPLACEMENT_CHAR_1}\r\n\r\n");
                let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_value_with_replacement_character_2() {
                let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{HEADER_NAME_REPLACEMENT_CHAR_2}\r\n\r\n");
                let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_header_value_with_multi_byte_utf8_character() {
                let request =
                    "GET https://example.com/test.json HTTP/1.1\r\nHeader-Name: Hello 😊\r\n\r\n";
                let url = parse_request_and_validate_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_fully_redacted_header_value() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                let url = parse_request_and_validate_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn request_fully_redacted_header_value_no_space_before_value() {
                let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                let url = parse_request_and_validate_redaction(request).unwrap();
                assert_eq!(url, "https://example.com/test.json");
            }

            #[test]
            fn response_no_header_redaction() {
                let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
                let body = parse_response_and_validate_redaction(response).unwrap().0;
                assert_eq!(body, "Hello, world!");
            }

            #[test]
            fn response_fully_redacted_header_value() {
                let response =
                    "HTTP/1.1 200 OK\r\nContent-Type: \0\0\0\0\0\0\0\0\0\0\r\n\r\nHello, world!";
                let body = parse_response_and_validate_redaction(response).unwrap().0;
                assert_eq!(body, "Hello, world!");
            }
        }

        mod fail {
            use super::*;

            mod request {
                use super::*;
                #[test]
                fn partially_redacted_header_value() {
                    let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/jso\0\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request).unwrap_err();
                    assert!(matches!(err, ParsingError::PartiallyRedactedHeaderValue));
                }

                #[test]
                fn partially_redacted_header_name() {
                    let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-typ\0: application/json\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request).unwrap_err();
                    assert!(matches!(err, ParsingError::RedactedHeaderName));
                }

                #[test]
                fn fully_redacted_header_name() {
                    let request = "GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0: application/json\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request).unwrap_err();
                    assert!(matches!(err, ParsingError::RedactedHeaderName));
                }

                #[test]
                fn fully_redacted_header_name_and_value() {
                    let request = "GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request).unwrap_err();
                    assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                }
            }

            mod response {
                use super::*;
                #[test]
                fn partially_redacted_header_value() {
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plai\0\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::PartiallyRedactedHeaderValue));
                }

                #[test]
                fn partially_redacted_header_name() {
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Typ\0: text/plain\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::RedactedHeaderName));
                }

                #[test]
                fn fully_redacted_header_name() {
                    let response =
                        "HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0: text/plain\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::RedactedHeaderName));
                }

                #[test]
                fn fully_redacted_header_name_and_value() {
                    let response =
                        "HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                }
            }
        }
    }
}
