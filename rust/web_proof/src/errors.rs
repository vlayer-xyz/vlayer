use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("No path in request")]
    NoPathInRequest,

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("Httparse error: {0}")]
    Httparse(#[from] httparse::Error),

    #[error("Partial httparse error")]
    Partial,

    #[error("Unsupported transfer encoding: {0}")]
    UnsupportedTransferEncoding(String),

    #[error("IO error: {0}")]
    StdIoError(#[from] std::io::Error),

    #[error("Header name is redacted")]
    RedactedName,

    #[error("Header value is partially redacted")]
    PartiallyRedactedValue,
}
