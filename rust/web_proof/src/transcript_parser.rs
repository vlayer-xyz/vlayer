use std::string::ToString;

use httparse::Header;

use crate::{
    redaction::{REDACTED_BYTE_CODE, RedactedTranscriptNameValue},
    utils::bytes::replace_bytes,
};

mod request;
mod response;

pub(crate) use request::parse_request_and_validate_redaction;
pub(crate) use response::parse_response_and_validate_redaction;

const MAX_HEADERS_NUMBER: usize = 40;
const CONTENT_TYPE: &str = "Content-Type";

fn replace_redacted_bytes(input: &[u8], replacement_char: char) -> Vec<u8> {
    replace_bytes(input, REDACTED_BYTE_CODE, replacement_char as u8)
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
                        let request = format!(
                            "GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR_PRIMARY}: application/json\r\n\r\n"
                        );
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_name_with_replacement_character_2() {
                        let request = format!(
                            "GET https://example.com/test.json HTTP/1.1\r\ncontent-type{REDACTION_REPLACEMENT_CHAR_SECONDARY}: application/json\r\n\r\n"
                        );
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_replacement_character_1() {
                        let request = format!(
                            "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR_PRIMARY}\r\n\r\n"
                        );
                        let url = parse_request_and_validate_redaction(request.as_bytes()).unwrap();
                        assert_eq!(url, "https://example.com/test.json");
                    }

                    #[test]
                    fn header_value_with_replacement_character_2() {
                        let request = format!(
                            "GET https://example.com/test.json HTTP/1.1\r\ncontent-type: application/json{REDACTION_REPLACEMENT_CHAR_SECONDARY}\r\n\r\n"
                        );
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

                mod header {
                    use super::*;

                    #[test]
                    fn no_header_redaction() {
                        let response =
                            b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n{}";
                        let body = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap();
                        assert_eq!(body, "{}");
                    }

                    #[test]
                    fn fully_redacted_header_value() {
                        let response =
                            b"HTTP/1.1 200 OK\r\nContent-Type: \0\0\0\0\0\0\0\0\0\0\r\n\r\n{}";
                        let body = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap();
                        assert_eq!(body, "{}");
                    }

                    #[test]
                    fn no_redaction_explicit_utf8_charset() {
                        let response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n{}";
                        let body = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap();
                        assert_eq!(body, "{}");
                    }
                }

                mod body {
                    use super::*;

                    #[test]
                    fn no_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\r\n"
                            + "\"string\": \"Hello, World!\",\r\n"
                            + "\"number\": 42,\r\n"
                            + "\"boolean\": true,\r\n"
                            + "\"array\": [1, 2, 3, \"four\"],\r\n"
                            + "\"object\": {\r\n"
                            + "\"nested_string\": \"Nested\",\r\n"
                            + "\"nested_number\": 99.99\r\n"
                            + "}\r\n"
                            + "}";
                        let body = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap();
                        assert_eq!(
                            body,
                            trim_start(
                                r#"{
                            "string": "Hello, World!",
                            "number": 42,
                            "boolean": true,
                            "array": [1, 2, 3, "four"],
                                "object": {
                                    "nested_string": "Nested",
                                    "nested_number": 99.99
                                }
                            }"#,
                            )
                        );
                    }

                    #[test]
                    fn blank_body() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "\r\n";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(
                            matches!(err, ParsingError::Json(err) if err.to_string() == "EOF while parsing a value at line 1 column 0")
                        );
                    }

                    #[test]
                    fn fully_redacted_string_value() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\r\n"
                            + "\"string\": \"\0\0\0\0\0\0\0\0\0\0\0\0\0\",\r\n"
                            + "\"number\": 42,\r\n"
                            + "\"boolean\": true,\r\n"
                            + "\"array\": [1, 2, 3, \"four\"],\r\n"
                            + "\"object\": {\r\n"
                            + "\"nested_string\": \"Nested\",\r\n"
                            + "\"nested_number\": 99.99\r\n"
                            + "}\r\n"
                            + "}";
                        let body = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::EnabledUnsafe,
                        )
                        .unwrap();
                        assert_eq!(
                            body,
                            trim_start(
                                r#"{
                            "string": "*************",
                            "number": 42,
                            "boolean": true,
                            "array": [1, 2, 3, "four"],
                            "object": {
                                "nested_string": "Nested",
                                "nested_number": 99.99
                            }
                            }"#,
                            )
                        );
                    }

                    #[test]
                    fn fully_redacted_nested_string_value() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\r\n"
                            + "\"string\": \"Hello, World!\",\r\n"
                            + "\"number\": 42,\r\n"
                            + "\"boolean\": true,\r\n"
                            + "\"array\": [1, 2, 3, \"four\"],\r\n"
                            + "\"object\": {\r\n"
                            + "\"nested_string\": \"\0\0\0\0\0\0\",\r\n"
                            + "\"nested_number\": 99.99\r\n"
                            + "}\r\n"
                            + "}";
                        let body = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::EnabledUnsafe,
                        )
                        .unwrap();
                        assert_eq!(
                            body,
                            trim_start(
                                r#"{
                            "string": "Hello, World!",
                            "number": 42,
                            "boolean": true,
                            "array": [1, 2, 3, "four"],
                                "object": {
                                    "nested_string": "******",
                                    "nested_number": 99.99
                                }
                            }"#,
                            )
                        );
                    }

                    #[test]
                    fn redact_string_value_inside_array() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "\r\n"
                            + "[{\"string\": \"\0\0\0\0\0\"}]";
                        let body = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::EnabledUnsafe,
                        )
                        .unwrap();
                        assert_eq!(body, trim_start(r#"[{"string": "*****"}]"#));
                    }
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

                mod header {
                    use super::*;

                    #[test]
                    fn partially_redacted_header_value() {
                        let response =
                            b"HTTP/1.1 200 OK\r\nContent-Type: text/plai\0\r\n\r\nHello, world!";
                        let err = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
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
                        let err = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseHeader, err_string) if err_string == "Content-Typ*: text/plain"
                        ));
                    }

                    #[test]
                    fn fully_redacted_header_name() {
                        let response =
                        b"HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0: text/plain\r\n\r\nHello, world!";
                        let err = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseHeader, err_string) if err_string == "************: text/plain"
                        ));
                    }

                    #[test]
                    fn fully_redacted_header_name_and_value() {
                        let response =
                        b"HTTP/1.1 200 OK\r\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\r\n\r\nHello, world!";
                        let err = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(err, ParsingError::Httparse(httparse::Error::HeaderName)));
                    }
                }
                mod body {
                    use super::*;

                    #[test]
                    fn number_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\r\n"
                            + "\"string\": \"Hello, World!\",\r\n"
                            + "\"number\": \0\0,\r\n"
                            + "\"boolean\": true,\r\n"
                            + "\"array\": [1, 2, 3, \"four\"],\r\n"
                            + "\"object\": {\r\n"
                            + "\"nested_string\": \"Nested\",\r\n"
                            + "\"nested_number\": 99.99\r\n"
                            + "}\r\n"
                            + "}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(
                            matches!(err, ParsingError::Json(err) if err.to_string() == "expected value at line 3 column 11")
                        );
                    }

                    #[test]
                    fn boolean_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\r\n"
                            + "\"string\": \"Hello, World!\",\r\n"
                            + "\"number\": 42,\r\n"
                            + "\"boolean\": \0\0\0\0,\r\n"
                            + "\"array\": [1, 2, 3, \"four\"],\r\n"
                            + "\"object\": {\r\n"
                            + "\"nested_string\": \"Nested\",\r\n"
                            + "\"nested_number\": 99.99\r\n"
                            + "}\r\n"
                            + "}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(err, ParsingError::Json(_)));
                    }

                    #[test]
                    fn key_partial_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\"string\0\": \"Hello\"}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseBody, err_string) if err_string == "$.string*: Hello"
                        ));
                    }

                    #[test]
                    fn invalid_json() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(
                            matches!(err, ParsingError::Json(err) if err.to_string() == "expected value at line 1 column 1")
                        );
                    }

                    #[test]
                    fn empty_body() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(
                            matches!(err, ParsingError::Json(err) if err.to_string() == "EOF while parsing a value at line 1 column 0")
                        );
                    }

                    #[test]
                    fn nested_key_partial_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\"object\": {\"nested_string\0\":\"Hello\"}}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseBody, err_string) if err_string == "$.object.nested_string*: Hello"
                        ));
                    }

                    #[test]
                    fn key_full_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\"\0\0\0\0\0\0\": \"Hello\"}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        println!("{err:?}");
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseBody, err_string) if err_string == "$.******: Hello"
                        ));
                    }

                    #[test]
                    fn key_full_redaction_for_empty_object() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\"\0\0\0\0\0\0\": {}}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseBody, err_string) if err_string == "$.******: "
                        ));
                    }

                    #[test]
                    fn key_full_redaction_for_empty_array() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\"\0\0\0\0\0\0\": []}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert!(matches!(
                            err,
                            ParsingError::RedactedName(RedactionElementType::ResponseBody, err_string) if err_string == "$.******: "
                        ));
                    }

                    #[test]
                    fn invalid_content_type() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: text/plain\r\n"
                            + "\r\n";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();

                        assert!(matches!(
                            err,
                            ParsingError::InvalidContentType(err_string) if err_string == "text/plain"
                        ));
                    }

                    #[test]
                    fn invalid_content_type_charset_utf16() {
                        let response = b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=UTF-16\r\n\r\n{}";

                        let err = parse_response_and_validate_redaction(
                            response,
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();

                        assert!(matches!(
                            err,
                            ParsingError::InvalidCharset(err_string) if err_string == "application/json; charset=UTF-16"
                        ));
                    }

                    #[test]
                    fn invalid_content_type_charset_iso() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json; charset=ISO-8859-1\r\n"
                            + "\r\n";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();

                        assert!(matches!(
                            err,
                            ParsingError::InvalidCharset(err_string) if err_string == "application/json; charset=ISO-8859-1"
                        ));
                    }

                    #[test]
                    fn fully_redacted_string_value_with_disabled_body_redaction() {
                        let response = "".to_string()
                            + "HTTP/1.1 200 OK\r\n"
                            + "Content-Type: application/json\r\n"
                            + "Content-Length: 136\r\n"
                            + "\r\n"
                            + "{\r\n"
                            + "\"string\": \"\0\0\0\0\0\0\0\0\0\0\0\0\0\",\r\n"
                            + "\"number\": 42,\r\n"
                            + "\"boolean\": true,\r\n"
                            + "\"array\": [1, 2, 3, \"four\"],\r\n"
                            + "\"object\": {\r\n"
                            + "\"nested_string\": \"Nested\",\r\n"
                            + "\"nested_number\": 99.99\r\n"
                            + "}\r\n"
                            + "}";
                        let err = parse_response_and_validate_redaction(
                            response.as_bytes(),
                            BodyRedactionMode::Disabled,
                        )
                        .unwrap_err();
                        assert_eq!(err, ParsingError::RedactionInResponseBody);
                    }
                }
            }
        }
    }

    fn trim_start(input: &str) -> String {
        input
            .lines()
            .map(str::trim_start)
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}
