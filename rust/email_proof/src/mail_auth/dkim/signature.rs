use crate::mail_auth::common::{crypto::Algorithm, verify::VerifySignature};

use super::Canonicalization;

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
