use std::string::FromUtf8Error;

use derivative::Derivative;
use thiserror::Error;

use crate::redaction::RedactionElementType;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq)]
pub enum ParsingError {
    #[error("No method in request")]
    NoMethodInRequest,

    #[error("Wrong method in request {0}")]
    WrongMethodInRequest(String),

    #[error("No host in request")]
    NoHostInRequest,

    #[error("Host is not a domain name")]
    HostIsNotADomainName,

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
    StdIoError(
        #[from]
        #[derivative(PartialEq = "ignore")]
        std::io::Error,
    ),

    #[error("Host name is redacted: {0}")]
    RedactedHost(String),

    #[error("{0} name is redacted: {1}")]
    RedactedName(RedactionElementType, String),

    #[error("{0} value is partially redacted: {1}")]
    PartiallyRedactedValue(RedactionElementType, String),

    #[error("Url parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Json parsing error: {0}")]
    Json(
        #[from]
        #[derivative(PartialEq = "ignore")]
        serde_json::Error,
    ),

    #[error("Invalid content-type: {0}")]
    InvalidContentType(String),

    #[error("Invalid charset: {0}")]
    InvalidCharset(String),

    #[error("Invalid mime type: {0}")]
    MimeParsing(
        #[from]
        #[derivative(PartialEq = "ignore")]
        mime::FromStrError,
    ),
}
