#![allow(dead_code)]

use std::fmt::Display;

use common::{crypto::HashAlgorithm, headers::Header};
use dkim::Canonicalization;
use resolver::Resolver;

pub mod common;
pub mod dkim;
pub(crate) mod resolver;

pub struct AuthenticatedMessage<'x> {
    pub headers: Vec<(&'x [u8], &'x [u8])>,
    pub dkim_headers: Vec<Header<'x, crate::mail_auth::Result<dkim::Signature>>>,
    pub body_hashes: Vec<(Canonicalization, HashAlgorithm, u64, Vec<u8>)>,
}

impl<'x> AuthenticatedMessage<'x> {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DkimResult {
    Pass,
    Neutral(crate::mail_auth::Error),
    Fail(crate::mail_auth::Error),
    PermError(crate::mail_auth::Error),
    TempError(crate::mail_auth::Error),
    None,
}

#[allow(dead_code)]
pub struct DkimOutput<'x> {
    result: DkimResult,
    signature: Option<&'x dkim::Signature>,
    report: Option<String>,
    // is_atps: bool,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    DkimRecord(String),
    FailedAuidMatch,
    FailedBodyHashMatch,
    SignatureExpired,
    DnsError(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SignatureExpired => write!(f, "Signature expired"),
            Error::FailedBodyHashMatch => {
                write!(f, "Calculated body hash does not match signature hash")
            }
            Error::FailedAuidMatch => write!(f, "AUID does not match domain name"),
            Error::DnsError(err) => write!(f, "DNS resolution error: {err}"),
            Error::DkimRecord(err) => write!(f, "Failed to parse DKIM record: {err}"),
        }
    }
}
impl std::error::Error for Error {}
