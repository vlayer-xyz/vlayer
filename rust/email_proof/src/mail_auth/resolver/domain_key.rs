use crate::mail_auth::{
    common::{
        crypto::{VerifyingKey, R_HASH_SHA256},
        verify::VerifySignature,
    },
    dkim::Canonicalization,
};

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

    pub fn has_flag(&self, flag: impl Into<u64>) -> bool {
        (self.f & flag.into()) != 0
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
