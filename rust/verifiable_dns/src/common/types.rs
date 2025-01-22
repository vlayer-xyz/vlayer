use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as};

use crate::{dns_over_https, verifier};

#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Signature(#[serde_as(as = "Base64")] pub Bytes);

#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PublicKey(#[serde_as(as = "Base64")] pub Bytes);

pub(crate) type Timestamp = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VerificationData {
    pub valid_until: Timestamp,
    pub signature: Signature,
    pub pub_key: PublicKey,
}

impl VerificationData {
    #[allow(dead_code)]
    pub fn verify_signature(
        &self,
        record: &dns_over_https::types::Record,
    ) -> Result<(), verifier::RecordVerifierError> {
        verifier::verify_signature(record, self.valid_until, &self.pub_key, &self.signature)
    }
}
