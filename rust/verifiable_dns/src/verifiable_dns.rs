pub(crate) mod resolver;
pub(crate) mod signer;
pub mod time;

pub use resolver::Resolver;
#[cfg(feature = "http")]
use time::RTClock;

#[cfg(feature = "http")]
use crate::{dns_over_https, dns_over_https::ExternalProvider, verifier};

#[cfg(feature = "http")]
pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;
