pub(crate) mod record;
mod resolver;
pub(crate) mod signer;
mod time;

use resolver::Resolver;
use serde::{Deserialize, Serialize};
use signer::{PublicKey, Signature};
use time::RTClock;

use crate::dns_over_https::provider::ExternalProvider;

pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;

type Timestamp = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VerificationData {
    pub valid_until: Timestamp,
    pub signature: Signature,
    pub pub_key: PublicKey,
}
