use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const TRACING_CONFIG: &str = "info";

pub(crate) fn init_tracing() {
    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(TRACING_CONFIG.into());
    let formatting_layer = tracing_subscriber::fmt::layer().with_ansi(true);
    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .init();
}
