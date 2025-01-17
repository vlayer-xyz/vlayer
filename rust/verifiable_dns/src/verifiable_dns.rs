pub(crate) mod record;
pub(crate) mod resolver;
pub(crate) mod signer;
pub(crate) mod time;

use resolver::Resolver;
use serde::{Deserialize, Serialize};
pub(crate) use signer::{PublicKey, Signature};
use time::RTClock;

use crate::{dns_over_https, dns_over_https::provider::ExternalProvider, verifier};

pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;

type Timestamp = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub(crate) struct VerificationData {
    pub(crate) valid_until: Timestamp,
    pub(crate) signature: Signature,
    pub(crate) pub_key: PublicKey,
}

impl VerificationData {
    pub fn verify_signature(
        &self,
        record: &dns_over_https::types::Record,
    ) -> Result<(), verifier::RecordVerifierError> {
        verifier::verify_signature(&record, self.valid_until, &self.pub_key, &self.signature)
    }
}
