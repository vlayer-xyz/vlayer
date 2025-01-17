pub(crate) mod record;
#[cfg(feature = "signer")]
mod resolver;
#[cfg(feature = "signer")]
pub(crate) mod signer;
#[cfg(feature = "signer")]
mod time;
pub(crate) mod types;

#[cfg(feature = "signer")]
use resolver::Resolver;
use serde::{Deserialize, Serialize};
#[cfg(feature = "http")]
use time::RTClock;

#[cfg(feature = "http")]
use crate::dns_over_https::ExternalProvider;
use crate::verifiable_dns::types::{PublicKey, Signature};

#[cfg(feature = "http")]
pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;

type Timestamp = u64;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct VerificationData {
    pub valid_until: Timestamp,
    pub signature: Signature,
    pub pub_key: PublicKey,
}
