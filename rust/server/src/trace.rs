use opentelemetry::sdk::trace::Tracer;
use opentelemetry::trace::TraceError;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const JAEGER_SERVICE_NAME: &str = "proving_server";
const TRACING_CONFIG: &str = "server=debug,tower_http=debug,axum::rejection=trace";

fn init_jaeger_tracer() -> Result<Tracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name(JAEGER_SERVICE_NAME)
        .install_simple()
}

pub(crate) fn init_tracing() -> Result<(), TraceError> {
    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or(TRACING_CONFIG.into());
    let formatting_layer = tracing_subscriber::fmt::layer();

    let jaeger_tracer = init_jaeger_tracer()?;
    let jaeger_layer = tracing_opentelemetry::layer().with_tracer(jaeger_tracer);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .with(jaeger_layer)
        .init();

    Ok(())
}
