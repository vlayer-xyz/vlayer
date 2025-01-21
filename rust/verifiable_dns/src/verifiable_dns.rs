pub(crate) mod resolver;
pub mod sign_record;
pub mod signer;
pub mod time;

pub use resolver::Resolver;
#[cfg(feature = "http")]
use time::RTClock;

#[cfg(feature = "http")]
use crate::dns_over_https::ExternalProvider;

#[cfg(feature = "http")]
pub type VerifiableDNSResolver = Resolver<RTClock, ExternalProvider, 2>;
