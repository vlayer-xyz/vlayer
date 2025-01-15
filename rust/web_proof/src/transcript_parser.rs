use std::{
    fmt::{self, Display, Formatter},
    iter::zip,
    str::from_utf8,
};

use httparse::{Header, Request, Response, Status, EMPTY_HEADER};
use url::Url;

use crate::errors::ParsingError;

const MAX_HEADERS_NUMBER: usize = 40;

const REDACTED_CHAR: char = '\0';

// Both '*' and '+' are valid header characters. Replacing redacted '\0' bytes with
// two different characters ensures the request is parsable and allows analysis
// of redacted content via diffs.
const REDACTION_REPLACEMENT_CHAR: char = '*';
const REDACTION_REPLACEMENT_DIFFERENT_CHAR: char = '+';

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct NameValue {
    pub(crate) name: String,
    pub(crate) value: Vec<u8>,
}

impl Display for NameValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {}",
            self.name,
            from_utf8(&self.value).unwrap_or(format!("{:?}", self.value).as_str())
        )
    }
}

pub(crate) fn parse_request_and_validate_redaction(request: &str) -> Result<String, ParsingError> {
    let request_string =
        request.replace(REDACTED_CHAR, REDACTION_REPLACEMENT_CHAR.to_string().as_str());
    let (path_with_replacement_1, headers_with_replacement_1) = parse_request(&request_string)?;

    let request_string =
        request.replace(REDACTED_CHAR, REDACTION_REPLACEMENT_DIFFERENT_CHAR.to_string().as_str());
    let (path_with_replacement_2, headers_with_replacement_2) = parse_request(&request_string)?;

    validate_name_value_redaction(
        &convert_headers(&headers_with_replacement_1),
        &convert_headers(&headers_with_replacement_2),
    )?;

    validate_name_value_redaction(
        &convert_path(&path_with_replacement_1)?,
        &convert_path(&path_with_replacement_2)?,
    )?;

    Ok(path_with_replacement_1)
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
) -> Result<(String, Vec<NameValue>), ParsingError> {
    let response_string =
        response.replace(REDACTED_CHAR, REDACTION_REPLACEMENT_CHAR.to_string().as_str());
    let (body_index, headers_with_replacement_1) = parse_response(&response_string)?;

    let response_string =
        response.replace(REDACTED_CHAR, REDACTION_REPLACEMENT_DIFFERENT_CHAR.to_string().as_str());
    let (_, headers_with_replacement_2) = parse_response(&response_string)?;

    validate_name_value_redaction(
        &convert_headers(&headers_with_replacement_1),
        &convert_headers(&headers_with_replacement_2),
    )?;

    let body = &response_string[body_index..];

    Ok((body.to_string(), convert_headers(&headers_with_replacement_1)))
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

fn validate_name_value_redaction(
    name_values_with_replacement_1: &[NameValue],
    name_values_with_replacement_2: &[NameValue],
) -> Result<(), ParsingError> {
    let zipped_pairs =
        zip(name_values_with_replacement_1.iter(), name_values_with_replacement_2.iter());

    let redacted_name = zipped_pairs.clone().find(|(l, r)| l.name != r.name);

    if let Some(pair) = redacted_name {
        return Err(ParsingError::RedactedName(pair.0.to_string()));
    }

    let partially_redacted_value = zipped_pairs
        .clone()
        .find(|(l, r)| !fully_redacted(&l.value, REDACTION_REPLACEMENT_CHAR) && l.value != r.value);

    if let Some(pair) = partially_redacted_value {
        return Err(ParsingError::PartiallyRedactedValue(pair.0.to_string()));
    }

    Ok(())
}

fn fully_redacted(input: &[u8], redacted_char: char) -> bool {
    input.iter().all(|&c| c == redacted_char as u8)
}

fn convert_headers(headers: &[Header]) -> Vec<NameValue> {
    headers
        .iter()
        .map(|header| NameValue {
            name: header.name.to_string(),
            value: header.value.to_vec(),
        })
        .collect()
}

fn convert_path(path: &str) -> Result<Vec<NameValue>, ParsingError> {
    Ok(Url::parse(path)?
        .query_pairs()
        .map(|param| NameValue {
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
                    NameValue {
                        name: "param1".to_string(),
                        value: "value1".to_string().into_bytes()
                    },
                    NameValue {
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
                    let request = "GET https://example.com/test.json?param=value HTTP/1.1\r\ncontent-type: application/json\r\n\r\n";
                    let url = parse_request_and_validate_redaction(request).unwrap();
                    assert_eq!(url, "https://example.com/test.json?param=value");
                }

                mod header {
                    use super::*;

                    #[test]
                    fn header_name_with_replacement_character_1() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR}: application/json\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_name_with_replacement_character_2() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_DIFFERENT_CHAR}: application/json\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_replacement_character_1() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR}\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_replacement_character_2() {
                        let request = format!("GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_DIFFERENT_CHAR}\r\n\r\n");
                        let url = parse_request_and_validate_redaction(request.as_str()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_multi_byte_utf8_character() {
                        let request =
                    "GET https://example.com/test.json HTTP/1.1\r\nHeader-Name: Hello 😊\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn fully_redacted_header_value() {
                        let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn fully_redacted_header_value_no_space_before_value() {
                        let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }
                }
                mod url {
                    use super::*;

                    #[test]
                    fn url_param_no_redaction() {
                        let request =
                            "GET https://example.com/test.json?param=value HTTP/1.1\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json?param=value");
                    }

                    #[test]
                    fn fully_redacted_url_param_value() {
                        let request =
                            "GET https://example.com/test.json?param=\0\0\0\0\0 HTTP/1.1\r\n\r\n";
                        let url = parse_request_and_validate_redaction(request).unwrap();
                        assert_eq!(url, "https://example.com/test.json?param=*****");
                    }

                    #[test]
                    fn fully_redacted_multiple_url_param_values() {
                        let request =
                            "GET https://example.com/test.json?param1=\0\0\0\0\0&param2=value2&param3=\0\0\0 HTTP/1.1\r\n\r\n";
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
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, world!";
                    let body = parse_response_and_validate_redaction(response).unwrap().0;
                    assert_eq!(body, "Hello, world!");
                }

                #[test]
                fn fully_redacted_header_value() {
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Type: \0\0\0\0\0\0\0\0\0\0\r\n\r\nHello, world!";
                    let body = parse_response_and_validate_redaction(response).unwrap().0;
                    assert_eq!(body, "Hello, world!");
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
                        let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/jso\0\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::PartiallyRedactedValue(_)));
                    }

                    #[test]
                    fn partially_redacted_header_name() {
                        let request = "GET https://example.com/test.json HTTP/1.1\r\ncontent-typ\0: application/json\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::RedactedName(_)));
                    }

                    #[test]
                    fn fully_redacted_header_name() {
                        let request = "GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0: application/json\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::RedactedName(_)));
                    }

                    #[test]
                    fn fully_redacted_header_name_and_value() {
                        let request = "GET https://example.com/test.json HTTP/1.1\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                    }
                }

                mod url {
                    use super::*;

                    #[test]
                    fn partially_redacted_url_param_value() {
                        let request =
                            "GET https://example.com/test.json?param1=value\0&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::PartiallyRedactedValue(_)));
                    }

                    #[test]
                    fn partially_redacted_url_param_name() {
                        let request =
                            "GET https://example.com/test.json?param\0=value1&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::RedactedName(_)));
                    }

                    #[test]
                    fn fully_redacted_url_param_name() {
                        let request =
                            "GET https://example.com/test.json?\0\0\0\0\0\0=value1&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::RedactedName(_)));
                    }

                    #[test]
                    fn fully_redacted_url_param_name_and_value() {
                        let request =
                            "GET https://example.com/test.json?\0\0\0\0\0\0\0\0\0\0\0\0&param2=value2 HTTP/1.1\r\n\r\n";
                        let err = parse_request_and_validate_redaction(request).unwrap_err();
                        assert!(matches!(err, ParsingError::RedactedName(_)));
                    }
                }
            }

            mod response {
                use super::*;
                #[test]
                fn partially_redacted_header_value() {
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Type: text/plai\0\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::PartiallyRedactedValue(_)));
                }

                #[test]
                fn partially_redacted_header_name() {
                    let response =
                        "HTTP/1.1 200 OK\r\nContent-Typ\0: text/plain\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::RedactedName(_)));
                }

                #[test]
                fn fully_redacted_header_name() {
                    let response =
                        "HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0: text/plain\r\n\r\nHello, world!";
                    let err = parse_response_and_validate_redaction(response).unwrap_err();
                    assert!(matches!(err, ParsingError::RedactedName(_)));
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
