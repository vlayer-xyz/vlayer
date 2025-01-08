mod record;
mod resolver;
mod signer;
mod time;

use resolver::Resolver;
use serde::Serialize;
use signer::{PublicKey, Signature};
use time::RTClock;

pub type VerifiableDNSResolver = Resolver<RTClock>;

type Timestamp = u64;
#[derive(Serialize, PartialEq, Debug)]
pub(crate) struct VerificationData {
    pub(crate) valid_until: Timestamp,
    pub(crate) signature: Signature,
    pub(crate) pub_key: PublicKey,
}
