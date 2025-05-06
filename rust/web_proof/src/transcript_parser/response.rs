use std::{io::Read, string::ToString};

use chunked_transfer::Decoder;
use httparse::{EMPTY_HEADER, Header, Response, Status};
use mime::Mime;

use super::{
    CONTENT_TYPE, MAX_HEADERS_NUMBER, REDACTED_BYTE_CODE, convert_headers, replace_redacted_bytes,
};
use crate::{
    errors::ParsingError,
    redaction::{
        REDACTION_REPLACEMENT_CHAR_PRIMARY, REDACTION_REPLACEMENT_CHAR_SECONDARY,
        RedactedTranscriptNameValue, RedactionElementType, validate_name_value_redaction,
    },
    utils::{bytes::all_match, json::json_to_redacted_transcript},
    web_proof::BodyRedactionMode,
};

pub(crate) fn parse_response_and_validate_redaction(
    response: &[u8],
    redaction_mode: BodyRedactionMode,
) -> Result<String, ParsingError> {
    let response_primary_replacement =
        replace_redacted_bytes(response, REDACTION_REPLACEMENT_CHAR_PRIMARY);
    let (body_primary_offset, headers_primary) = parse_response(&response_primary_replacement)?;

    let response_secondary_replacement =
        replace_redacted_bytes(response, REDACTION_REPLACEMENT_CHAR_SECONDARY);
    let (body_secondary_offset, headers_secondary) =
        parse_response(&response_secondary_replacement)?;

    validate_name_value_redaction(
        &convert_headers(&headers_primary),
        &convert_headers(&headers_secondary),
        RedactionElementType::ResponseHeader,
    )?;

    validate_content_type_and_charset(&headers_primary)?;

    let body_primary = &response_primary_replacement[body_primary_offset..];
    let body_secondary = &response_secondary_replacement[body_secondary_offset..];

    let body_primary = handle_chunked_transfer_encoding(
        &convert_headers(&headers_primary),
        &String::from_utf8(body_primary.to_vec())?,
    )?;
    let body_secondary = handle_chunked_transfer_encoding(
        &convert_headers(&headers_secondary),
        &String::from_utf8(body_secondary.to_vec())?,
    )?;

    validate_name_value_redaction(
        &json_to_redacted_transcript(&body_primary)?,
        &json_to_redacted_transcript(&body_secondary)?,
        RedactionElementType::ResponseBody,
    )?;

    if redaction_mode == BodyRedactionMode::Disabled {
        if body_primary_offset != body_secondary_offset {
            return Err(ParsingError::RedactionInResponseBody);
        }
        let original_body = &response[body_primary_offset..];
        if original_body.contains(&REDACTED_BYTE_CODE) {
            return Err(ParsingError::RedactionInResponseBody);
        }
    }

    Ok(body_primary)
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

fn validate_content_type_and_charset(headers: &[Header]) -> Result<(), ParsingError> {
    let content_type_header = headers
        .iter()
        .find(|header| header.name.eq_ignore_ascii_case(CONTENT_TYPE));

    if let Some(header) = content_type_header {
        let content_type = String::from_utf8_lossy(header.value).to_string();
        if all_match(content_type.as_bytes(), REDACTION_REPLACEMENT_CHAR_PRIMARY as u8) {
            return Ok(());
        }

        let mime: Mime = content_type.parse()?;

        if mime.type_() != "application" || mime.subtype() != "json" {
            return Err(ParsingError::InvalidContentType(content_type));
        }

        if let Some(charset) = mime.get_param("charset") {
            if !charset.as_str().eq_ignore_ascii_case("utf-8") {
                return Err(ParsingError::InvalidCharset(content_type));
            }
        }
    }
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    mod redaction {
        use super::*;

        mod success {
            use super::*;

            mod header {
                use super::*;

                #[test]
                fn no_header_redaction() {
                    let response = b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\n\r\n{}";
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

        mod fail {
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

    fn trim_start(input: &str) -> String {
        input
            .lines()
            .map(str::trim_start)
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}
