use std::string::ToString;

use httparse::{EMPTY_HEADER, Header, Request};
use url::Url;

use super::{MAX_HEADERS_NUMBER, convert_headers, replace_redacted_bytes};
use crate::{
    errors::ParsingError,
    redaction::{
        REDACTION_REPLACEMENT_CHAR_PRIMARY, REDACTION_REPLACEMENT_CHAR_SECONDARY,
        RedactedTranscriptNameValue, RedactionElementType, validate_name_value_redaction,
    },
};

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

fn convert_path(path: &str) -> Result<Vec<RedactedTranscriptNameValue>, ParsingError> {
    Ok(Url::parse(path)?
        .query_pairs()
        .map(|param| RedactedTranscriptNameValue {
            name: param.0.to_string(),
            value: param.1.to_string().into_bytes(),
        })
        .collect())
}
