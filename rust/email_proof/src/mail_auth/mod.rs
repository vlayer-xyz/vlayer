#![allow(dead_code)]

use common::{crypto::HashAlgorithm, headers::Header};
use dkim::Canonicalization;
use error::Error;
use resolver::Resolver;

pub mod common;
pub mod dkim;
pub(crate) mod error;
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

pub type Result<T> = std::result::Result<T, Error>;
