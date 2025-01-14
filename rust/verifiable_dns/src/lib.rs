mod dns_over_https;

mod verifiable_dns;

pub use dns_over_https::{ExternalProvider, Provider, Query, Response, MIME_DNS_JSON_CONTENT_TYPE};
pub use verifiable_dns::VerifiableDNSResolver;
