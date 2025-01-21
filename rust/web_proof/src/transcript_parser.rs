use std::{io::Read, string::ToString};

use chunked_transfer::Decoder;
use httparse::{Header, Request, Response, Status, EMPTY_HEADER};
use url::Url;

use crate::{
    errors::ParsingError,
    redaction::{
        validate_name_value_redaction, RedactedTranscriptNameValue, RedactionElementType,
        REDACTED_BYTE_CODE, REDACTION_REPLACEMENT_CHAR_PRIMARY,
        REDACTION_REPLACEMENT_CHAR_SECONDARY,
    },
    utils::bytes::replace_bytes,
};

const MAX_HEADERS_NUMBER: usize = 40;

pub(crate) fn parse_request_and_validate_redaction(request: &[u8]) -> Result<String, ParsingError> {
    let request_primary_replacement =
        replace_redacted_bytes(request, REDACTION_REPLACEMENT_CHAR_PRIMARY);
    let (path_primary, headers_primary) = parse_request(&request_primary_replacement)?;

    let request_secondary_replacement =
        replace_redacted_bytes(request, REDACTION_REPLACEMENT_CHAR_SECONDARY);
    let (path_secondary, headers_secondary) = parse_request(&request_secondary_replacement)?;

    validate_name_value_redaction(
        &convert_headers(&headers_primary),
        &convert_headers(&headers_secondary),
        RedactionElementType::RequestHeader,
    )?;

    validate_name_value_redaction(
        &convert_path(&path_primary)?,
        &convert_path(&path_secondary)?,
        RedactionElementType::RequestUrlParam,
    )?;

    Ok(path_primary)
}

fn parse_request(request: &[u8]) -> Result<(String, [Header; MAX_HEADERS_NUMBER]), ParsingError> {
    let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
    let mut req = Request::new(&mut headers);
    req.parse(request)?;

    let path = req.path.ok_or(ParsingError::NoPathInRequest)?.to_string();

    Ok((path, headers))
}

pub(crate) fn parse_response_and_validate_redaction(
    response: &[u8],
) -> Result<String, ParsingError> {
    let response_primary_replacement =
        replace_redacted_bytes(response, REDACTION_REPLACEMENT_CHAR_PRIMARY);
    let (body_primary, headers_primary) = parse_response(&response_primary_replacement)?;

    let response_secondary_replacement =
        replace_redacted_bytes(response, REDACTION_REPLACEMENT_CHAR_SECONDARY);
    let (_body_secondary, headers_secondary) = parse_response(&response_secondary_replacement)?;

    validate_name_value_redaction(
        &convert_headers(&headers_primary),
        &convert_headers(&headers_secondary),
        RedactionElementType::ResponseHeader,
    )?;

    let body_primary = &response_primary_replacement[body_primary..];

    let body_primary = handle_chunked_transfer_encoding(
        &convert_headers(&headers_primary),
        &String::from_utf8(body_primary.to_vec())?,
    )?;

    Ok(body_primary)
}

fn replace_redacted_bytes(input: &[u8], replacement_char: char) -> Vec<u8> {
    replace_bytes(input, REDACTED_BYTE_CODE, replacement_char as u8)
}

fn handle_chunked_transfer_encoding(
    headers: &[RedactedTranscriptNameValue],
    body: &str,
) -> Result<String, ParsingError> {
    let transfer_encoding_header = headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case("Transfer-Encoding"));

    match transfer_encoding_header {
        Some(header) if header.value.eq_ignore_ascii_case(b"chunked") => {
            let mut decoder = Decoder::new(body.as_bytes());
            let mut decoded_body = String::new();
            decoder.read_to_string(&mut decoded_body)?;
            Ok(decoded_body)
        }
        Some(header) if header.value.eq_ignore_ascii_case(b"identity") => Ok(body.to_string()),
        Some(header) => Err(ParsingError::UnsupportedTransferEncoding(
            String::from_utf8_lossy(header.value.as_ref()).to_string(),
        )),
        None => Ok(body.to_string()),
    }
}

fn parse_response(response: &[u8]) -> Result<(usize, [Header; MAX_HEADERS_NUMBER]), ParsingError> {
    let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
    let mut res = Response::new(&mut headers);
    let body_index = match res.parse(response)? {
        Status::Complete(t) => t,
        Status::Partial => return Err(ParsingError::Partial),
    };

    Ok((body_index, headers))
}

fn convert_headers(headers: &[Header]) -> Vec<RedactedTranscriptNameValue> {
    headers
        .iter()
        .map(|header| RedactedTranscriptNameValue {
            name: header.name.to_string(),
            value: header.value.to_vec(),
        })
        .collect()
}

fn convert_path(path: &str) -> Result<Vec<RedactedTranscriptNameValue>, ParsingError> {
    Ok(Url::parse(path)?
        .query_pairs()
        .map(|param| RedactedTranscriptNameValue {
            name: param.0.to_string(),
            value: param.1.to_string().into_bytes(),
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod convert_path {
        use super::*;

        #[test]
        fn success() {
            let name_values =
                convert_path("https://example.com/test.json?param1=value1&param2=value2").unwrap();
            assert_eq!(
                name_values,
                vec![
                    RedactedTranscriptNameValue {
                        name: "param1".to_string(),
                        value: "value1".to_string().into_bytes()
                    },
                    RedactedTranscriptNameValue {
                        name: "param2".to_string(),
                        value: "value2".to_string().into_bytes()
                    }
                ]
            );
        }

        #[test]
        fn fail() {
            let err = convert_path("https://").unwrap_err();
            assert!(matches!(err, ParsingError::UrlParse(url::ParseError::EmptyHost)));
        }
    }

    mod redaction {
        use super::*;

        mod success {
            use super::*;

            mod request {
                use super::*;

                #[test]
                fn no_redaction() {
                    let request = b"GET https://example.com/test.json?param=value HTTP/1.1\r\ncontent-type: application/json\r\n\r\n";
                    let url = parse_request_and_validate_redaction(request).unwrap();
                    assert_eq!(url, "https://example.com/test.json?param=value");
                }

                mod header {
                    use super::*;

                    #[test]
                    fn header_name_with_replacement_character_1() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR_PRIMARY}: application/json\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_name_with_replacement_character_2() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR_SECONDARY}: application/json\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_replacement_character_1() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR_PRIMARY}\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_replacement_character_2() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR_SECONDARY}\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn fully_redacted_header_value() {
                        let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-type: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n{}\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn fully_redacted_header_value_no_space_before_value() {
                        let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-type:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }
                }
                mod url {
                    use super::*;

                    #[test]
                    fn url_param_no_redaction() {
                        let request =
                            b"GET https://example.com/test.json?param=value HTTP/1.1\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json?param=value");
                    }

                    #[test]
                    fn fully_redacted_url_param_value() {
                        let request =
                            b"GET https://example.com/test.json?param=\0\0\0\0\0 HTTP/1.1\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json?param=*****");
                    }

                    #[test]
                    fn fully_redacted_multiple_url_param_values() {
                        let request =
                            b"GET https://example.com/test.json?param1=\0\0\0\0\0&param2=value2&param3=\0\0\0 HTTP/1.1\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(
                            url,
                            "https://example.com/test.json?param1=*****&param2=value2&param3=***"
                        );
                    }
                }
            }

            mod response {
                use super::*;

                #[test]
                fn no_header_redaction() {
                    let response = b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n{}";
                    let body = parse_response_and_validate_redaction(response).unwrap();
                    assert_eq!(body, "{}");
                }

                #[test]
                fn fully_redacted_header_value() {
                    let response =
                        b"HTTP/1.1 200 OK\r\nContent-Type: \0\0\0\0\0\0\0\0\0\0\r\n\r\n{}";
                    let body = parse_response_and_validate_redaction(response).unwrap();
                    assert_eq!(body, "{}");
                }
            }
        }

        mod fail {
            use super::*;

            mod request {
                use super::*;

                mod header {
                    use super::*;

                    #[test]
                    fn partially_redacted_header_value() {
                        let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/jso\0\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(
                            matches!(err, ParsingError::PartiallyRedactedValue(RedactionElementType::RequestHeader, err_string) if err_string == "content-type: application/jso*")
                        );
                    }

                    #[test]
                    fn partially_redacted_header_name() {
                        let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-typ\0: application/json\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::RequestHeader, err_string) if err_string == "content-typ*: application/json"
                        ));
                    }

                    #[test]
                    fn fully_redacted_header_name() {
                        let request = b"GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0: application/json\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::RequestHeader, err_string) if err_string == "************: application/json"
                        ));
                    }

                    #[test]
                    fn fully_redacted_header_name_and_value() {
                        let request = b"GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                    }
                }

                mod url {
                    use super::*;

                    #[test]
                    fn partially_redacted_url_param_value() {
                        let request =
                            b"GET https://example.com/test.json?param1=value\0&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::PartiallyRedactedValue(
                                RedactionElementType::RequestUrlParam,
                                err_string
                            ) if err_string == "param1: value*"
                        ));
                    }

                    #[test]
                    fn partially_redacted_url_param_name() {
                        let request =
                            b"GET https://example.com/test.json?param\0=value1&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::RequestUrlParam, err_string) if err_string == "param*: value1"
                        ));
                    }

                    #[test]
                    fn fully_redacted_url_param_name() {
                        let request =
                            b"GET https://example.com/test.json?\0\0\0\0\0\0=value1&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::RequestUrlParam, err_string) if err_string == "******: value1"
                        ));
                    }

                    #[test]
                    fn fully_redacted_url_param_name_and_value() {
                        let request =
                            b"GET https://example.com/test.json?\0\0\0\0\0\0\0\0\0\0\0\0&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::RequestUrlParam, err_string) if err_string == "************: "
                        ));
                    }
                }
            }

            mod response {
                use super::*;
                #[test]
                fn partially_redacted_header_value() {
                    let response =
                        b"HTTP/1.1 200 OK\r\nContent-Type: text/plai\0\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::PartiallyRedactedValue(
                            RedactionElementType::ResponseHeader,
                            err_string
                        ) if err_string == "Content-Type: text/plai*"
                    ));
                }

                #[test]
                fn partially_redacted_header_name() {
                    let response =
                        b"HTTP/1.1 200 OK\r\nContent-Typ\0: text/plain\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::ResponseHeader, err_string) if err_string == "Content-Typ*: text/plain"
                    ));
                }

                #[test]
                fn fully_redacted_header_name() {
                    let response =
                        b"HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0: text/plain\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::ResponseHeader, err_string) if err_string == "************: text/plain"
                    ));
                }

                #[test]
                fn fully_redacted_header_name_and_value() {
                    let response =
                        b"HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                }
            }
        }
    }
}
