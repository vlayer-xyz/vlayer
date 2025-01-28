mod record;
mod resolver;
mod signer;
mod time;

use resolver::Resolver;
use serde::{Deserialize, Serialize};
use signer::{PublicKey, Signature};
use time::RTClock;

use crate::dns_over_https::provider::ExternalProvider;

pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;

type Timestamp = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub(crate) struct VerificationData {
    pub(crate) valid_until: Timestamp,
    pub(crate) signature: Signature,
    pub(crate) pub_key: PublicKey,
}
