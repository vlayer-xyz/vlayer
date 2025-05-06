use std::string::FromUtf8Error;

use derivative::Derivative;
use http::method::InvalidMethod;
use thiserror::Error;

use crate::redaction::RedactionElementType;

#[derive(Error, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub enum ParsingError {
    #[error("No HTTP method in request")]
    NoHttpMethodInRequest,

    #[error("No path in request")]
    NoPathInRequest,

    #[error("Malformed request. No newline")]
    MalformedRequestNoNewline,

    #[error("Malformed request. First line mismatch after reconstruction")]
    MalformedRequestReconstructionMismatch,

    #[error("Invalid HTTP method")]
    InvalidHttpMethod(
        #[from]
        #[derivative(PartialEq = "ignore")]
        InvalidMethod,
    ),

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

    #[error(
        "Response body contains redacted elements. This is unsafe. Please set the BodyRedactionMode to Enabled_UNSAFE to allow this"
    )]
    RedactionInResponseBody,

    #[error(
        "Redaction in first line of the request is forbidden when UrlTestMode is set to Full. Use Prefix to allow redaction"
    )]
    RedactionInFirstLine,

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
