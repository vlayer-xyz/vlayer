use crate::mail_auth::dkim::Canonicalization;

use super::crypto::{Algorithm, VerifyingKey};

pub struct DomainKey {
    pub p: Box<dyn VerifyingKey>,
    pub f: u64,
}
impl DomainKey {
    pub(crate) fn verify<'a>(
        &self,
        headers: &mut dyn Iterator<Item = (&'a [u8], &'a [u8])>,
        input: &impl VerifySignature,
        canonicalization: Canonicalization,
    ) -> crate::mail_auth::Result<()> {
        self.p
            .verify(headers, input.signature(), canonicalization, input.algorithm())
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
