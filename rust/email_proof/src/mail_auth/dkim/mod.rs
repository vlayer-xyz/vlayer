use super::{
    common::{
        crypto::{Algorithm, HashAlgorithm},
        verify::VerifySignature,
    },
    DkimOutput, DkimResult, Error,
};

// pub mod builder;
// pub mod canonicalize;
// pub mod headers;
pub mod parse;
// pub mod sign;
pub mod verify;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Canonicalization {
    #[default]
    Relaxed,
    Simple,
}

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

impl<'x> DkimOutput<'x> {
    pub(crate) fn perm_err(err: Error) -> Self {
        DkimOutput {
            result: DkimResult::PermError(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) fn temp_err(err: Error) -> Self {
        DkimOutput {
            result: DkimResult::TempError(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) fn fail(err: Error) -> Self {
        DkimOutput {
            result: DkimResult::Fail(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) fn neutral(err: Error) -> Self {
        DkimOutput {
            result: DkimResult::Neutral(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) fn dns_error(err: Error) -> Self {
        if matches!(&err, Error::DnsError(_)) {
            DkimOutput::temp_err(err)
        } else {
            DkimOutput::perm_err(err)
        }
    }

    pub(crate) fn with_signature(mut self, signature: &'x Signature) -> Self {
        self.signature = signature.into();
        self
    }
}
