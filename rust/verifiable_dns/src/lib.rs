mod dns_over_https;

mod types;
#[cfg(feature = "signer")]
mod verifiable_dns;

#[cfg(feature = "http")]
pub use dns_over_https::ExternalProvider;
pub use dns_over_https::{
    types::{Record as DNSRecord, RecordType},
    Provider, Query, Response, MIME_DNS_JSON_CONTENT_TYPE,
};
pub use types::{PublicKey, Signature, VerificationData};
#[cfg(feature = "http")]
pub use verifiable_dns::VerifiableDNSResolver;
