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
    headers.iter().map(Into::into).collect()
}
