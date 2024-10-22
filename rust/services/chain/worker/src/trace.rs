use tracing_subscriber::{
    fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

const TRACING_CONFIG: &str = "info";

pub(crate) fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(TRACING_CONFIG.into());
    let formatting_layer = layer().with_ansi(true);
    registry().with(env_filter).with(formatting_layer).init();
}
