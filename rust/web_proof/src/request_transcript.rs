use std::string::FromUtf8Error;

use httparse::{Request, EMPTY_HEADER};
use thiserror::Error;
use tlsn_core::RedactedTranscript;

const MAX_HEADERS_NUMBER: usize = 40;

pub(crate) struct RequestTranscript {
    transcript: RedactedTranscript,
}

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("No path in request")]
    NoPathInRequest(),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Httparse error: {0}")]
    Httparse(#[from] httparse::Error),
}

impl RequestTranscript {
    pub(crate) fn parse_url(self) -> Result<String, ParsingError> {
        let request_string = String::from_utf8(self.transcript.data().to_vec())?;

        let mut headers = [EMPTY_HEADER; MAX_HEADERS_NUMBER];
        let mut req = Request::new(&mut headers);
        req.parse(request_string.as_bytes())?;

        let url = req.path.ok_or(ParsingError::NoPathInRequest())?.to_string();
        Ok(url)
    }
}
