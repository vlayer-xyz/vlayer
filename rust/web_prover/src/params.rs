use std::{collections::BTreeMap, str, sync::Arc};

use derive_builder::Builder;
use derive_more::Debug;
use derive_new::new;
pub use hyper::http::Method;
use rangeset::RangeSet;
use tlsn_core::transcript::Transcript;

use crate::RedactionConfig;

#[derive(Debug, Clone, new, Default)]
pub struct NotaryConfig {
    /// Notary host (domain name or IP)
    pub host: String,
    /// Notary port
    pub port: u16,
    /// Notary API path
    pub path_prefix: String,
    /// Whether to use TLS for notary connection
    pub enable_tls: bool,
}

#[derive(Builder, Clone, Debug)]
#[builder(setter(into))]
pub struct NotarizeParams {
    pub notary_config: NotaryConfig,
    pub server_domain: String,
    pub server_host: String,
    pub server_port: u16,
    pub uri: String,
    #[builder(default)]
    pub method: Method,
    #[builder(setter(custom))]
    pub headers: BTreeMap<String, String>,
    #[builder(setter(into), default)]
    pub body: Vec<u8>,
    #[builder(
        setter(custom, strip_option),
        default = "default_redaction_config_fn()"
    )]
    #[debug(skip)]
    pub redaction_config_fn: RedactionConfigFn,
    #[builder(default = "1 << 12")]
    pub max_sent_data: usize,
    #[builder(default = "1 << 14")]
    pub max_recv_data: usize,
}

impl NotarizeParamsBuilder {
    pub fn headers(
        &mut self,
        headers: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> &mut Self {
        self.headers = Some(
            headers
                .into_iter()
                .map(|(name, value)| (name.as_ref().to_string(), value.as_ref().to_string()))
                .collect(),
        );
        self
    }
}

pub type RedactionConfigFn = Arc<dyn Fn(&Transcript) -> RedactionConfig + Send + Sync>;

impl NotarizeParamsBuilder {
    pub fn redaction_config_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&Transcript) -> RedactionConfig + Send + Sync + 'static,
    {
        self.redaction_config_fn = Some(Arc::new(f));
        self
    }
}

fn default_redaction_config_fn() -> RedactionConfigFn {
    Arc::new(|transcript: &Transcript| RedactionConfig {
        sent: RangeSet::from(0..transcript.sent().len()),
        recv: RangeSet::from(0..transcript.received().len()),
    })
}
