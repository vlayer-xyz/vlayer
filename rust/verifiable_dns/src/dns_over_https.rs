pub(crate) mod provider;
pub(crate) mod types;

pub use provider::{ExternalProvider, Provider};
pub use types::{Query, Response};

pub const MIME_DNS_JSON_CONTENT_TYPE: &str = "application/dns-json";
