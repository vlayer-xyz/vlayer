use std::sync::Arc;

use tracing_subscriber::{
    fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt, EnvFilter,
};

use crate::LogFormat;

const DEFAULT_RUST_LOG: &str = "info";

pub fn init_tracing(log_format: LogFormat, secrets: Option<Vec<String>>) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(DEFAULT_RUST_LOG.into());
    let registry = registry().with(env_filter);
    let secrets = Arc::new(secrets.unwrap_or_default());

    match log_format {
        LogFormat::Json => {
            let formatting_layer = layer()
                .with_writer(move || RedactingWriter::new(std::io::stdout(), secrets.clone()))
                .json();
            registry.with(formatting_layer).init();
        }
        LogFormat::Plain => {
            let formatting_layer = layer()
                .with_writer(move || RedactingWriter::new(std::io::stdout(), secrets.clone()))
                .with_ansi(true);
            registry.with(formatting_layer).init();
        }
    }
}

struct RedactingWriter<W: std::io::Write> {
    inner: W,
    secrets: Arc<Vec<String>>,
}

impl<W: std::io::Write> RedactingWriter<W> {
    fn new(inner: W, secrets: Arc<Vec<String>>) -> Self {
        Self { inner, secrets }
    }
}

impl<W: std::io::Write> std::io::Write for RedactingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut output = String::from_utf8_lossy(buf).to_string();
        for secret in self.secrets.iter() {
            let placeholder = "*".repeat(secret.len());
            output = output.replace(secret, &placeholder);
        }
        self.inner.write(output.as_bytes())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
