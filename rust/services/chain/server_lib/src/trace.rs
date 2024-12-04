use common::LogFormat;
use tracing_subscriber::{
    fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

const DEFAULT_RUST_LOG: &str = "info";

pub fn init_tracing(log_format: Option<LogFormat>) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(DEFAULT_RUST_LOG.into());
    let registry = registry().with(env_filter);
    if log_format == Some(LogFormat::Json) {
        let formatting_layer = layer().json();
        registry.with(formatting_layer).init();
    } else {
        let formatting_layer = layer().with_ansi(true);
        registry.with(formatting_layer).init();
    }
}
