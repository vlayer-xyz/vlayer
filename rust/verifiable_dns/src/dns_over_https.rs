#[cfg(feature = "http")]
mod external_provider;
pub(crate) mod provider;
pub(crate) mod types;

#[cfg(feature = "http")]
pub use external_provider::ExternalProvider;
pub use provider::Provider;
pub use types::{Query, Response};

pub const MIME_DNS_JSON_CONTENT_TYPE: &str = "application/dns-json";
