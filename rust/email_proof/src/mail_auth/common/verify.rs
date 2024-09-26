use crate::mail_auth::dkim::Canonicalization;

use super::crypto::{Algorithm, VerifyingKey, R_HASH_SHA256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DomainKey {
    pub p: Box<VerifyingKey>,
    pub f: u64,
}
impl DomainKey {
    pub(crate) fn verify<'a>(
        &self,
        headers: &dyn Iterator<Item = (&'a [u8], &'a [u8])>,
        input: &impl VerifySignature,
        canonicalization: Canonicalization,
    ) -> crate::mail_auth::Result<()> {
        self.p
            .verify(headers, input.signature(), canonicalization, input.algorithm())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DkimRecordError {
    Err,
}

impl TryFrom<String> for DomainKey {
    type Error = DkimRecordError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let domain_key = Self {
            p: Box::new(VerifyingKey::RsaKey(value.as_bytes().into())),
            f: R_HASH_SHA256,
        };
        Ok(domain_key)
    }
}

pub trait VerifySignature {
    fn selector(&self) -> &str;

    fn domain(&self) -> &str;

    fn signature(&self) -> &[u8];

    fn algorithm(&self) -> Algorithm;

    fn domain_key(&self) -> String {
        let s = self.selector();
        let d = self.domain();
        let mut key = String::with_capacity(s.len() + d.len() + 13);
        key.push_str(s);
        key.push_str("._domainkey.");
        key.push_str(d);
        key.push('.');
        key
    }
}
