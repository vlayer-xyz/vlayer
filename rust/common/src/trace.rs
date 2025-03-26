use tracing_subscriber::{
    EnvFilter, fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt,
};

use crate::LogFormat;

const DEFAULT_RUST_LOG: &str = "info";

pub fn init_tracing(log_format: LogFormat) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(DEFAULT_RUST_LOG.into());
    let registry = registry().with(env_filter);
    match log_format {
        LogFormat::Json => {
            let formatting_layer = layer().json();
            registry.with(formatting_layer).init();
        }
        LogFormat::Plain => {
            let formatting_layer = layer().with_ansi(true);
            registry.with(formatting_layer).init();
        }
    }
}
