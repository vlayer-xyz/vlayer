use http::HeaderName;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub(crate) enum _RequestLine<'a> {
    Header { name: &'a str, value: &'a str },
    Other(&'a str),
}

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Header not found {0}")]
    _HeaderNotFound(String),

    #[error("Capture error")]
    _Capture,
}

#[derive(Debug)]
pub(crate) struct _RequestParseResult<'a> {
    lines: Vec<_RequestLine<'a>>,
}

impl _RequestParseResult<'_> {
    pub(crate) fn _header(&self, header_name: HeaderName) -> Result<&str, ParserError> {
        self.lines
            .iter()
            .filter_map(|line| {
                if let _RequestLine::Header { name, value } = line {
                    if *name == header_name {
                        return Some(*value);
                    }
                }
                None
            })
            .next()
            .ok_or(ParserError::_HeaderNotFound(header_name.to_string()))
    }
}

pub(crate) fn _parse_web_proof_request(request: &str) -> Result<_RequestParseResult, ParserError> {
    request
        .lines()
        .map(_parse_web_proof_request_line)
        .collect::<Vec<Result<_RequestLine, ParserError>>>()
        .into_iter()
        .collect::<Result<Vec<_RequestLine>, ParserError>>()
        .map(|lines| _RequestParseResult { lines })
}

pub(crate) fn _parse_web_proof_request_line(line: &str) -> Result<_RequestLine, ParserError> {
    let header_regex = Regex::new(r"^\s*(\S+)\s*:\s*(\S+)\s*$")?;

    if let Some(captures) = header_regex.captures(line) {
        let name = captures.get(1).ok_or(ParserError::_Capture)?.as_str();
        let value = captures.get(2).ok_or(ParserError::_Capture)?.as_str();
        Ok(_RequestLine::Header { name, value })
    } else {
        Ok(_RequestLine::Other(line))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixtures::read_fixture;
    use http::header;
    use std::collections::HashMap;

    #[test]
    fn hidden_as_other() {
        assert_eq!(
            _RequestLine::Other(""),
            _parse_web_proof_request_line("").unwrap()
        );
        assert_eq!(
            _RequestLine::Other("X"),
            _parse_web_proof_request_line("X").unwrap()
        );
        assert_eq!(
            _RequestLine::Other("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            _parse_web_proof_request_line("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap()
        );
    }

    #[test]
    fn http_method_and_url_as_other() {
        let method_and_url_line = "GET https://example.com HTTP/1.1";

        assert_eq!(
            _RequestLine::Other(method_and_url_line),
            _parse_web_proof_request_line(method_and_url_line).unwrap()
        );
    }

    #[test]
    fn http_header() {
        assert_eq!(
            _RequestLine::Header {
                name: "host",
                value: "example.com"
            },
            _parse_web_proof_request_line("host: example.com").unwrap()
        );

        assert_eq!(
            _RequestLine::Header {
                name: "connection",
                value: "close"
            },
            _parse_web_proof_request_line("connection: close").unwrap()
        );
    }

    #[test]
    fn empty_header_name_as_other() {
        assert_eq!(
            _RequestLine::Other(": example.com"),
            _parse_web_proof_request_line(": example.com").unwrap()
        );
    }

    #[test]
    fn empty_header_value_as_other() {
        assert_eq!(
            _RequestLine::Other("host: "),
            _parse_web_proof_request_line("host: ").unwrap()
        );
    }

    #[test]
    fn extract_header_values_from_fixture() {
        let request = read_fixture("./testdata/sent_request.txt");

        for (header, expected_value) in HashMap::from([
            (header::HOST, Ok("api.x.com")),
            (header::CONNECTION, Ok("close")),
            (
                header::ACCEPT,
                Err(ParserError::_HeaderNotFound(header::ACCEPT.to_string())),
            ),
        ]) {
            assert_eq!(
                expected_value,
                _parse_web_proof_request(&request).unwrap()._header(header)
            );
        }
    }
}
