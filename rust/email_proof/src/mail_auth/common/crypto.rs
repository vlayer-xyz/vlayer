use crate::mail_auth::{dkim::Canonicalization, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerifyingKey {
    RsaKey(Vec<u8>),
}

#[allow(unused_variables)]
#[allow(clippy::unused_self)]
impl VerifyingKey {
    pub fn verify<'a>(
        &self,
        headers: &dyn Iterator<Item = (&'a [u8], &'a [u8])>,
        signature: &[u8],
        canonicalication: Canonicalization,
        algorithm: Algorithm,
    ) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub enum HashAlgorithm {
    Sha1 = R_HASH_SHA1,
    Sha256 = R_HASH_SHA256,
}

impl HashAlgorithm {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Algorithm {
    RsaSha1,
    #[default]
    RsaSha256,
    Ed25519Sha256,
}

impl From<Algorithm> for HashAlgorithm {
    fn from(a: Algorithm) -> Self {
        match a {
            Algorithm::RsaSha256 | Algorithm::Ed25519Sha256 => HashAlgorithm::Sha256,
            Algorithm::RsaSha1 => HashAlgorithm::Sha1,
        }
    }
}

pub(crate) const R_HASH_SHA1: u64 = 0x01;
pub(crate) const R_HASH_SHA256: u64 = 0x02;
