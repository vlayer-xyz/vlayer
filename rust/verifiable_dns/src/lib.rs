mod dns_over_https;

mod common;
#[cfg(feature = "signer")]
pub mod verifiable_dns;

#[cfg(feature = "http")]
pub use dns_over_https::ExternalProvider;
#[allow(dead_code)]
mod verifier;

pub use common::types::{PublicKey, Signature, VerificationData};
pub use dns_over_https::{
    MIME_DNS_JSON_CONTENT_TYPE, Provider, Query, Response,
    types::{Record as DNSRecord, RecordType},
};
#[cfg(feature = "http")]
pub use verifiable_dns::VerifiableDNSResolver;
pub use verifier::RecordVerifierError;
