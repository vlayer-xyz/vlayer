use super::common::{crypto::HashAlgorithm, headers::Header};
use super::dkim::{Canonicalization, Signature};

pub struct AuthenticatedMessage<'x> {
    pub headers: Vec<(&'x [u8], &'x [u8])>,
    pub dkim_headers: Vec<Header<'x, crate::mail_auth::Result<Signature>>>,
    pub body_hashes: Vec<(Canonicalization, HashAlgorithm, u64, Vec<u8>)>,
}

impl<'x> AuthenticatedMessage<'x> {}
