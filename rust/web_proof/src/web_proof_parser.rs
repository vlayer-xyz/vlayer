use http::HeaderName;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),

    #[error("Header not found {0}")]
    HeaderNotFound(String),

    #[error("Capture error")]
    Capture,
}

#[derive(Debug)]
pub(crate) struct RequestParseResult<'a> {
    lines: Vec<RequestLine<'a>>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum RequestLine<'a> {
    Header { name: &'a str, value: &'a str },
    Other(&'a str),
}

impl RequestParseResult<'_> {
    pub(crate) fn header(&self, header_name: HeaderName) -> Result<&str, ParserError> {
        self.lines
            .iter()
            .filter_map(|line| {
                if let RequestLine::Header { name, value } = line {
                    if *name == header_name {
                        return Some(*value);
                    }
                }
                None
            })
            .next()
            .ok_or(ParserError::HeaderNotFound(header_name.to_string()))
    }
}

// TODO: Consider using `httparse` crate for HTTP parsing, but first research how redaction works in TLSN
// to ensure that integrating this library will properly parse redacted HTTP request/response.
pub(crate) fn parse_web_proof_request(request: &str) -> Result<RequestParseResult, ParserError> {
    request
        .lines()
        .map(parse_web_proof_request_line)
        .collect::<Vec<Result<RequestLine, ParserError>>>()
        .into_iter()
        .collect::<Result<Vec<RequestLine>, ParserError>>()
        .map(|lines| RequestParseResult { lines })
}

fn parse_web_proof_request_line(line: &str) -> Result<RequestLine, ParserError> {
    let header_regex = Regex::new(r"^\s*(\S+)\s*:\s*(\S+)\s*$")?;

    if let Some(captures) = header_regex.captures(line) {
        let name = captures.get(1).ok_or(ParserError::Capture)?.as_str();
        let value = captures.get(2).ok_or(ParserError::Capture)?.as_str();
        Ok(RequestLine::Header { name, value })
    } else {
        Ok(RequestLine::Other(line))
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
            RequestLine::Other(""),
            parse_web_proof_request_line("").unwrap()
        );
        assert_eq!(
            RequestLine::Other("X"),
            parse_web_proof_request_line("X").unwrap()
        );
        assert_eq!(
            RequestLine::Other("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            parse_web_proof_request_line("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX").unwrap()
        );
    }

    #[test]
    fn http_method_and_url_as_other() {
        let method_and_url_line = "GET https://example.com HTTP/1.1";

        assert_eq!(
            RequestLine::Other(method_and_url_line),
            parse_web_proof_request_line(method_and_url_line).unwrap()
        );
    }

    #[test]
    fn http_header() {
        assert_eq!(
            RequestLine::Header {
                name: "host",
                value: "example.com"
            },
            parse_web_proof_request_line("host: example.com").unwrap()
        );

        assert_eq!(
            RequestLine::Header {
                name: "connection",
                value: "close"
            },
            parse_web_proof_request_line("connection: close").unwrap()
        );
    }

    #[test]
    fn empty_header_name_as_other() {
        assert_eq!(
            RequestLine::Other(": example.com"),
            parse_web_proof_request_line(": example.com").unwrap()
        );
    }

    #[test]
    fn empty_header_value_as_other() {
        assert_eq!(
            RequestLine::Other("host: "),
            parse_web_proof_request_line("host: ").unwrap()
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
                Err(ParserError::HeaderNotFound(header::ACCEPT.to_string())),
            ),
        ]) {
            assert_eq!(
                expected_value,
                parse_web_proof_request(&request).unwrap().header(header)
            );
        }
    }
}
