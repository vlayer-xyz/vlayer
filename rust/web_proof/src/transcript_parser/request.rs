use std::{str::FromStr, string::ToString};

use derive_new::new;
use httparse::{EMPTY_HEADER, Request};
use url::Url;

use super::{MAX_HEADERS_NUMBER, REDACTED_BYTE_CODE, convert_headers, replace_redacted_bytes};
use crate::{
    errors::ParsingError,
    redaction::{
        REDACTION_REPLACEMENT_CHAR_PRIMARY, REDACTION_REPLACEMENT_CHAR_SECONDARY,
        RedactedTranscriptNameValue, RedactionElementType, validate_name_value_redaction,
    },
    web_proof::UrlTestMode,
};

#[derive(Debug, Eq, PartialEq, new)]
pub struct ParsedRequest {
    /// Such as `GET`.
    pub method: http::Method,
    /// Such as `https://example.com/path`.
    pub url: String,
    /// Such as `1` for `HTTP/1.1`.
    pub minor_http_version: u8,
    pub headers: Vec<RedactedTranscriptNameValue>,
}

impl ParsedRequest {
    pub fn first_line(&self) -> String {
        format!("{} {} HTTP/1.{}", self.method, self.url, self.minor_http_version)
    }
}

pub(crate) fn parse_request_and_validate_redaction(
    request: &[u8],
    url_test_mode: UrlTestMode,
) -> Result<String, ParsingError> {
    let request_primary_replacement =
        replace_redacted_bytes(request, REDACTION_REPLACEMENT_CHAR_PRIMARY);
    let request_primary = parse_request(&request_primary_replacement)?;

    let request_secondary_replacement =
        replace_redacted_bytes(request, REDACTION_REPLACEMENT_CHAR_SECONDARY);
    let request_secondary = parse_request(&request_secondary_replacement)?;

    let first_line = split_first_line(request)?;
    if url_test_mode == UrlTestMode::Full && first_line.contains(&REDACTED_BYTE_CODE) {
        return Err(ParsingError::RedactionInFirstLine);
    }
    if split_first_line(&request_primary_replacement)? != request_primary.first_line().as_bytes()
        || split_first_line(&request_secondary_replacement)?
            != request_secondary.first_line().as_bytes()
    {
        return Err(ParsingError::MalformedRequestReconstructionMismatch);
    }

    validate_name_value_redaction(
        &request_primary.headers,
        &request_secondary.headers,
        RedactionElementType::RequestHeader,
    )?;

    validate_name_value_redaction(
        &convert_path(&request_primary.url)?,
        &convert_path(&request_secondary.url)?,
        RedactionElementType::RequestUrlParam,
    )?;

    Ok(request_primary.url)
}

fn split_first_line(request: &[u8]) -> Result<&[u8], ParsingError> {
    let first_newline_position = request
        .windows(2)
        .position(|n: &[u8]| n == "\r\n".as_bytes())
        .ok_or(ParsingError::MalformedRequestNoNewline)?;
    let (first_line, _) = request.split_at(first_newline_position);
    Ok(first_line)
}

fn parse_request(request: &[u8]) -> Result<ParsedRequest, ParsingError> {
    let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
    let mut req = Request::new(&mut headers);
    req.parse(request)?;

    let method_str = req
        .method
        .ok_or(ParsingError::NoHttpMethodInRequest)?
        .to_string();
    let method = http::Method::from_str(&method_str)?;
    let url = req.path.ok_or(ParsingError::NoPathInRequest)?.to_string();
    let version = req.version.ok_or(ParsingError::NoPathInRequest)?;
    let headers = convert_headers(req.headers);

    Ok(ParsedRequest::new(method, url, version, headers))
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

            #[test]
            fn no_redaction() {
                let request = b"GET https://example.com/test.json?param=value HTTP/1.1\r\ncontent-type: application/json\r\n\r\n";
                let url = parse_request_and_validate_redaction(request, UrlTestMode::Full).unwrap();
                assert_eq!(url, "https://example.com/test.json?param=value");
            }

            mod header {
                use super::*;

                #[test]
                fn header_name_with_replacement_character_1() {
                    let request = format!(
                        "GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR_PRIMARY}: application/json\r\n\r\n"
                    );
                    let url =
                        parse_request_and_validate_redaction(request.as_bytes(), UrlTestMode::Full)
                            .unwrap();
                    assert_eq!(url, "https://example.com/test.json");
                }

                #[test]
                fn header_name_with_replacement_character_2() {
                    let request = format!(
                        "GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR_SECONDARY}: application/json\r\n\r\n"
                    );
                    let url =
                        parse_request_and_validate_redaction(request.as_bytes(), UrlTestMode::Full)
                            .unwrap();
                    assert_eq!(url, "https://example.com/test.json");
                }

                #[test]
                fn header_value_with_replacement_character_1() {
                    let request = format!(
                        "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR_PRIMARY}\r\n\r\n"
                    );
                    let url =
                        parse_request_and_validate_redaction(request.as_bytes(), UrlTestMode::Full)
                            .unwrap();
                    assert_eq!(url, "https://example.com/test.json");
                }

                #[test]
                fn header_value_with_replacement_character_2() {
                    let request = format!(
                        "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR_SECONDARY}\r\n\r\n"
                    );
                    let url =
                        parse_request_and_validate_redaction(request.as_bytes(), UrlTestMode::Full)
                            .unwrap();
                    assert_eq!(url, "https://example.com/test.json");
                }

                #[test]
                fn fully_redacted_header_value() {
                    let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-type: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n{}\r\n";
                    let url =
                        parse_request_and_validate_redaction(request, UrlTestMode::Full).unwrap();
                    assert_eq!(url, "https://example.com/test.json");
                }

                #[test]
                fn fully_redacted_header_value_no_space_before_value() {
                    let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-type:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                    let url =
                        parse_request_and_validate_redaction(request, UrlTestMode::Full).unwrap();
                    assert_eq!(url, "https://example.com/test.json");
                }
            }
            mod url {
                use super::*;

                #[test]
                fn url_param_no_redaction() {
                    let request = b"GET https://example.com/test.json?param=value HTTP/1.1\r\n\r\n";
                    let url =
                        parse_request_and_validate_redaction(request, UrlTestMode::Full).unwrap();
                    assert_eq!(url, "https://example.com/test.json?param=value");
                }

                #[test]
                fn fully_redacted_url_param_value() {
                    let request =
                        b"GET https://example.com/test.json?param=\0\0\0\0\0 HTTP/1.1\r\n\r\n";
                    let url =
                        parse_request_and_validate_redaction(request, UrlTestMode::Prefix).unwrap();
                    assert_eq!(url, "https://example.com/test.json?param=*****");
                }

                #[test]
                fn fully_redacted_multiple_url_param_values() {
                    let request =
                            b"GET https://example.com/test.json?param1=\0\0\0\0\0&param2=value2&param3=\0\0\0 HTTP/1.1\r\n\r\n";
                    let url =
                        parse_request_and_validate_redaction(request, UrlTestMode::Prefix).unwrap();
                    assert_eq!(
                        url,
                        "https://example.com/test.json?param1=*****&param2=value2&param3=***"
                    );
                }
            }
        }

        mod fail {
            use super::*;

            mod header {
                use super::*;

                #[test]
                fn partially_redacted_header_value() {
                    let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/jso\0\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Full)
                        .unwrap_err();
                    assert!(
                        matches!(err, ParsingError::PartiallyRedactedValue(RedactionElementType::RequestHeader, err_string) if err_string == "content-type: application/jso*")
                    );
                }

                #[test]
                fn partially_redacted_header_name() {
                    let request = b"GET https://example.com/test.json HTTP/1.1\r\ncontent-typ\0: application/json\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Full)
                        .unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::RequestHeader, err_string) if err_string == "content-typ*: application/json"
                    ));
                }

                #[test]
                fn fully_redacted_header_name() {
                    let request = b"GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0: application/json\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Full)
                        .unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::RequestHeader, err_string) if err_string == "************: application/json"
                    ));
                }

                #[test]
                fn fully_redacted_header_name_and_value() {
                    let request = b"GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Full)
                        .unwrap_err();
                    assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                }
            }

            mod url {
                use super::*;

                #[test]
                fn partially_redacted_url_param_value() {
                    let request =
                            b"GET https://example.com/test.json?param1=value\0&param2=value2 HTTP/1.1\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Prefix)
                        .unwrap_err();
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
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Prefix)
                        .unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::RequestUrlParam, err_string) if err_string == "param*: value1"
                    ));
                }

                #[test]
                fn fully_redacted_url_param_name() {
                    let request =
                            b"GET https://example.com/test.json?\0\0\0\0\0\0=value1&param2=value2 HTTP/1.1\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Prefix)
                        .unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::RequestUrlParam, err_string) if err_string == "******: value1"
                    ));
                }

                #[test]
                fn fully_redacted_url_param_name_and_value() {
                    let request =
                            b"GET https://example.com/test.json?\0\0\0\0\0\0\0\0\0\0\0\0&param2=value2 HTTP/1.1\r\n\r\n";
                    let err = parse_request_and_validate_redaction(request, UrlTestMode::Prefix)
                        .unwrap_err();
                    assert!(matches!(
                        err,
                        ParsingError::RedactedName(RedactionElementType::RequestUrlParam, err_string) if err_string == "************: "
                    ));
                }
            }
        }
    }
}
