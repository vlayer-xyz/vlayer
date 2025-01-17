pub(crate) mod record;
mod resolver;
pub(crate) mod signer;
mod time;

use resolver::Resolver;
#[cfg(feature = "http")]
use time::RTClock;

#[cfg(feature = "http")]
use crate::dns_over_https::ExternalProvider;

#[cfg(feature = "http")]
pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;
