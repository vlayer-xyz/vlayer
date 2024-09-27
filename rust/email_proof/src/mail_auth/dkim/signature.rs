use crate::mail_auth::{
    common::{crypto::Algorithm, verify::VerifySignature},
    resolver::domain_key::DomainKey,
};

use super::{Canonicalization, Flag};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Signature {
    //     pub v: u32,
    pub a: Algorithm,
    pub d: String,
    pub s: String,
    pub b: Vec<u8>,
    pub bh: Vec<u8>,
    pub h: Vec<String>,
    //     pub z: Vec<String>,
    pub i: String,
    pub l: u64,
    pub x: u64,
    pub t: u64,
    pub ch: Canonicalization,
    pub cb: Canonicalization,
}

impl VerifySignature for Signature {
    fn signature(&self) -> &[u8] {
        &self.b
    }

    fn algorithm(&self) -> Algorithm {
        self.a
    }

    fn selector(&self) -> &str {
        &self.s
    }

    fn domain(&self) -> &str {
        &self.d
    }
}

impl Signature {
    #[allow(clippy::while_let_on_iterator)]
    pub(crate) fn validate_auid(&self, record: &DomainKey) -> bool {
        // Enforce t=s flag
        if !self.i.is_empty() && record.has_flag(Flag::MatchDomain) {
            let mut auid = self.i.chars();
            let mut domain = self.d.chars();
            while let Some(ch) = auid.next() {
                if ch == '@' {
                    break;
                }
            }
            while let Some(ch) = auid.next() {
                if let Some(dch) = domain.next() {
                    if ch != dch {
                        return false;
                    }
                } else {
                    break;
                }
            }
            if domain.next().is_some() {
                return false;
            }
        }

        true
    }
}
