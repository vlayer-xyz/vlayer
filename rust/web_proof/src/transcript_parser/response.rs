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
