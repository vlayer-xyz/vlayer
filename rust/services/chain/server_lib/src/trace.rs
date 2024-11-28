use tracing_subscriber::{
    fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

const DEFAULT_RUST_LOG: &str = "info";

pub fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(DEFAULT_RUST_LOG.into());
    let formatting_layer = layer().with_ansi(true);
    registry().with(env_filter).with(formatting_layer).init();
}
