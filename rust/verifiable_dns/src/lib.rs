mod dns_over_https;

mod verifiable_dns;

pub use dns_over_https::{Provider, Query, Response};
pub use verifiable_dns::VerifiableDNSResolver;
