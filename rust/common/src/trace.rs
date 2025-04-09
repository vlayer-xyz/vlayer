use std::{
    io::{Result, Write, stdout},
    sync::Arc,
};

use derive_new::new;
use tracing_subscriber::{
    EnvFilter, Layer, fmt::layer, layer::SubscriberExt, registry, util::SubscriberInitExt,
};

use crate::LogFormat;

const DEFAULT_RUST_LOG: &str = "info";

pub fn init_tracing(log_format: LogFormat, secrets: Vec<String>) {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(DEFAULT_RUST_LOG.into());
    let registry = registry().with(env_filter);
    let secrets = Arc::new(secrets);

    let redacting_writer = move || RedactingWriter::new(stdout(), secrets.clone());
    let formatting_layer = match log_format {
        LogFormat::Json => layer().with_writer(redacting_writer).json().boxed(),
        LogFormat::Plain => layer()
            .with_writer(redacting_writer)
            .with_ansi(true)
            .boxed(),
    };
    registry.with(formatting_layer).init();
}

#[derive(new)]
struct RedactingWriter<W: Write> {
    inner: W,
    secrets: Arc<Vec<String>>,
}

impl<W: Write> Write for RedactingWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut output = String::from_utf8_lossy(buf).to_string();
        for secret in self.secrets.iter() {
            let placeholder = "*".repeat(secret.len());
            output = output.replace(secret, &placeholder);
        }
        self.inner.write(output.as_bytes())
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
