use super::common::crypto::{Algorithm, HashAlgorithm};

use signature::Signature;

pub mod output;
pub mod parse;
pub mod result;
pub mod signature;
pub mod verify;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Canonicalization {
    #[default]
    Relaxed,
    Simple,
}

pub(crate) const R_FLAG_MATCH_DOMAIN: u64 = 0x20;

#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(u64)]
pub(crate) enum Flag {
    // Testing = R_FLAG_TESTING,
    MatchDomain = R_FLAG_MATCH_DOMAIN,
}

impl From<Flag> for u64 {
    fn from(v: Flag) -> Self {
        v as u64
    }
}

impl From<Algorithm> for HashAlgorithm {
    fn from(a: Algorithm) -> Self {
        match a {
            Algorithm::RsaSha256 | Algorithm::Ed25519Sha256 => HashAlgorithm::Sha256,
            Algorithm::RsaSha1 => HashAlgorithm::Sha1,
        }
    }
}
