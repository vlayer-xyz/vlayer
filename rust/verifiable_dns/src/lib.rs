mod dns_over_https;

mod verifiable_dns;

#[cfg(feature = "http")]
pub use dns_over_https::ExternalProvider;
pub use dns_over_https::{
    types::RecordType, Provider, Query, Response, MIME_DNS_JSON_CONTENT_TYPE,
};
#[cfg(feature = "http")]
pub use verifiable_dns::VerifiableDNSResolver;
pub use verifiable_dns::{
    record::Record as DNSRecord,
    types::{PublicKey, Signature},
    VerificationData,
};
