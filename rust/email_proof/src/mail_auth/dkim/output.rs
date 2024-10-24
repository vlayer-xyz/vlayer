use super::{result::Result, Signature};
use crate::mail_auth::Error;

#[allow(dead_code)]
pub struct Output<'x> {
    result: Result,
    signature: Option<&'x Signature>,
    report: Option<String>,
    // is_atps: bool,
}

impl<'x> Output<'x> {
    pub(crate) const fn perm_err(err: Error) -> Self {
        Self {
            result: Result::PermError(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) const fn temp_err(err: Error) -> Self {
        Self {
            result: Result::TempError(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) const fn fail(err: Error) -> Self {
        Self {
            result: Result::Fail(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) const fn neutral(err: Error) -> Self {
        Self {
            result: Result::Neutral(err),
            signature: None,
            report: None,
        }
    }

    pub(crate) const fn dns_error(err: Error) -> Self {
        if matches!(&err, Error::Dns(_)) {
            Self::temp_err(err)
        } else {
            Self::perm_err(err)
        }
    }

    pub(crate) fn with_signature(mut self, signature: &'x Signature) -> Self {
        self.signature = signature.into();
        self
    }
}
