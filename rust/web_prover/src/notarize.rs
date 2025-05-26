use std::str;

use anyhow::{Context, Result};
use derive_more::{Deref, From, Into};
use http_body_util::{BodyExt, Full};
use hyper::{
    Request as HttpRequest, StatusCode,
    body::Bytes,
    header::{self, HeaderName},
};
use hyper_util::rt::TokioIo;
use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use tlsn_common::config::ProtocolConfig;
use tlsn_core::{
    CryptoProvider, Secrets, attestation::Attestation, request::RequestConfig,
    transcript::TranscriptCommitConfig,
};
use tlsn_prover::{Prover, ProverConfig};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use tracing::debug;

use crate::{Method, NotarizeParams, RedactionConfig};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";

pub async fn notarize(params: NotarizeParams) -> Result<(Attestation, Secrets, RedactionConfig)> {
    debug!("notarizing...");

    let NotarizeParams {
        notary_config,
        server_domain,
        server_host,
        server_port,
        uri,
        headers,
        body,
        method,
        redaction_config_fn,
        max_sent_data,
        max_recv_data,
    } = params;

    let mut notary_client_builder = NotaryClient::builder();

    notary_client_builder
        .host(notary_config.host)
        .port(notary_config.port)
        .path_prefix(notary_config.path_prefix)
        .enable_tls(notary_config.enable_tls);

    #[cfg(feature = "tlsn-jwt")]
    #[cfg(not(clippy))]
    if let Some(jwt) = notary_config.jwt {
        notary_client_builder.jwt(jwt);
    }

    let notary_client = notary_client_builder.build()?;

    debug!("preparing notarization request...");

    let notarization_request = NotarizationRequest::builder()
        .max_sent_data(max_sent_data)
        .max_recv_data(max_recv_data)
        .build()?;

    let Accepted {
        io: notary_connection,
        id: _session_id,
        ..
    } = notary_client
        .request_notarization(notarization_request)
        .await
        .context("Could not connect to notary. Make sure it is running.")?;

    debug!("preparing notarization request done");

    let prover_config = ProverConfig::builder()
        .server_name(server_domain.as_ref())
        .protocol_config(
            ProtocolConfig::builder()
                .max_sent_data(max_sent_data)
                .max_recv_data(max_recv_data)
                .build()?,
        )
        .crypto_provider(CryptoProvider::default())
        .build()?;

    debug!("initializing prover...");

    let prover = Prover::new(prover_config)
        .setup(notary_connection.compat())
        .await?;

    debug!("connecting to client on '{server_host}:{server_port}'");

    let client_socket = tokio::net::TcpStream::connect((server_host, server_port)).await?;

    debug!("setting up MPC-TLS connection...");

    let (mpc_tls_connection, prover_fut) = prover.connect(client_socket.compat()).await?;
    let mpc_tls_connection = TokioIo::new(mpc_tls_connection.compat());

    let prover_task = tokio::spawn(prover_fut);

    let (mut request_sender, connection) =
        hyper::client::conn::http1::handshake(mpc_tls_connection).await?;

    tokio::spawn(connection);

    debug!("preparing request...");

    let request = prepare_request(&server_domain, &uri, &headers, method, body)?;

    debug!("starting an MPC TLS connection with the server");

    let response = request_sender.send_request(request.into()).await?;

    debug!("got a response from the server: {}", response.status());

    let status = response.status();
    if status != StatusCode::OK {
        let body = response.collect().await?.to_bytes();
        let body = String::from_utf8_lossy(&body);
        anyhow::bail!(
            "Failed to notarize: server responded with status '{status}', body: '{body}'",
        );
    }

    let prover = prover_task.await??;

    let mut prover = prover.start_notarize();

    let transcript = prover.transcript();

    debug!("transcript sent: {:?}", str::from_utf8(transcript.sent()));
    debug!("transcript received: {:?}", str::from_utf8(transcript.received()));

    let mut builder = TranscriptCommitConfig::builder(transcript);

    let redaction_config = redaction_config_fn(transcript);

    builder.commit_sent(&redaction_config.sent)?;
    builder.commit_recv(&redaction_config.recv)?;

    prover.transcript_commit(builder.build()?);

    let request_config = RequestConfig::default();

    let (attestation, secrets) = Box::pin(prover.finalize(&request_config)).await?;

    debug!("finished notarizing");

    Ok((attestation, secrets, redaction_config))
}

fn prepare_request(
    server_domain: &str,
    uri: &str,
    headers: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    method: Method,
    body: impl AsRef<[u8]>,
) -> Result<Request> {
    let mut request_builder = HttpRequest::builder().uri(uri).method(method);

    let header_map = request_builder
        .headers_mut()
        .ok_or_else(|| anyhow::anyhow!("could not extract headers from RequestBuilder"))?;

    for (k, v) in headers {
        let header_name = HeaderName::from_lowercase(k.as_ref().to_lowercase().as_bytes())?;
        header_map.append(header_name, v.as_ref().parse()?);
    }

    for (name, value) in [
        (header::HOST, server_domain),
        (header::ACCEPT, "*/*"),
        (header::ACCEPT_ENCODING, "identity"),
        (header::CONNECTION, "close"),
        (header::USER_AGENT, USER_AGENT),
    ] {
        header_map.entry(name).or_insert(value.parse()?);
    }

    let request: Request = request_builder
        .body(Full::new(Bytes::from(body.as_ref().to_vec())))
        .map(Into::into)?;

    debug!("{request:#?}");

    Ok(request)
}

#[derive(From, Into, Deref)]
struct Request(HttpRequest<Full<Bytes>>);

impl std::fmt::Debug for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "uri: {}", self.0.uri())?;
        writeln!(f, "method: {}", self.0.method())?;
        writeln!(f, "headers:")?;
        for (name, value) in self.0.headers() {
            writeln!(f, "  {name}: {value:#?}")?;
        }
        write!(f, "body: {:?}", self.0.body())
    }
}

#[cfg(test)]
mod tests {
    use hyper::header::HeaderValue;

    use super::*;

    #[test]
    fn should_correctly_prepare_request_body() {
        let request = prepare_request(
            "lotr-api.online",
            "/auth_header_require?param1=value1&param2=value2",
            [("Authorization", "s3cret_t0ken")],
            Method::GET,
            "abc",
        )
        .unwrap();

        let body = request.body();

        assert_eq!(r#"Full { data: Some(b"abc") }"#, format!("{body:?}"));
    }

    #[test]
    fn should_not_set_default_headers_if_specified_by_the_user() {
        let request = prepare_request(
            "lotr-api.online",
            "/",
            [("User-Agent", "curl/1.0.0")],
            Method::GET,
            "",
        )
        .unwrap();

        let headers: Vec<(&HeaderName, &HeaderValue)> = request
            .headers()
            .iter()
            .filter(|&(name, _)| name == header::USER_AGENT)
            .collect();

        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0].1, "curl/1.0.0");
    }
}
