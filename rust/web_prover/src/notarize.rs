use std::{collections::HashMap, str};

use anyhow::{Context, Result};
use http_body_util::Full;
use hyper::{Request, StatusCode, body::Bytes};
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

use crate::{NotarizeParams, RedactionConfig};

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
const MAX_SENT_DATA: usize = 1 << 12;
const MAX_RECV_DATA: usize = 1 << 14;

pub async fn notarize(params: NotarizeParams) -> Result<(Attestation, Secrets, RedactionConfig)> {
    let NotarizeParams {
        notary_config,
        server_domain,
        server_host,
        server_port,
        uri,
        headers,
        body,
        redaction_config_fn,
    } = params;

    let notary_client = NotaryClient::builder()
        .host(notary_config.host)
        .port(notary_config.port)
        .path_prefix(notary_config.path_prefix)
        .enable_tls(notary_config.enable_tls)
        .build()?;

    let notarization_request = NotarizationRequest::builder()
        .max_sent_data(MAX_SENT_DATA)
        .max_recv_data(MAX_RECV_DATA)
        .build()?;

    let Accepted {
        io: notary_connection,
        id: _session_id,
        ..
    } = notary_client
        .request_notarization(notarization_request)
        .await
        .context("Could not connect to notary. Make sure it is running.")?;

    let prover_config = ProverConfig::builder()
        .server_name(server_domain.as_ref())
        .protocol_config(
            ProtocolConfig::builder()
                .max_sent_data(MAX_SENT_DATA)
                .max_recv_data(MAX_RECV_DATA)
                .build()?,
        )
        .crypto_provider(CryptoProvider::default())
        .build()?;

    let prover = Prover::new(prover_config)
        .setup(notary_connection.compat())
        .await?;

    let client_socket = tokio::net::TcpStream::connect((server_host, server_port)).await?;

    let (mpc_tls_connection, prover_fut) = prover.connect(client_socket.compat()).await?;
    let mpc_tls_connection = TokioIo::new(mpc_tls_connection.compat());

    let prover_task = tokio::spawn(prover_fut);

    let (mut request_sender, connection) =
        hyper::client::conn::http1::handshake(mpc_tls_connection).await?;

    tokio::spawn(connection);

    let request = prepare_request(&server_domain, &uri, &headers, body)?;

    debug!("Starting an MPC TLS connection with the server");

    let response = request_sender.send_request(request).await?;

    debug!("Got a response from the server: {}", response.status());

    assert!(response.status() == StatusCode::OK);

    let prover = prover_task.await??;

    let mut prover = prover.start_notarize();

    let transcript = prover.transcript();

    debug!("Transcript sent: {:?}", str::from_utf8(transcript.sent()));
    debug!("Transcript received: {:?}", str::from_utf8(transcript.received()));

    let mut builder = TranscriptCommitConfig::builder(transcript);

    let redaction_config = redaction_config_fn(transcript);

    builder.commit_sent(&redaction_config.sent)?;
    builder.commit_recv(&redaction_config.recv)?;

    prover.transcript_commit(builder.build()?);

    let request_config = RequestConfig::default();

    let (attestation, secrets) = Box::pin(prover.finalize(&request_config)).await?;
    debug!("Finished notarizing");
    Ok((attestation, secrets, redaction_config))
}

fn prepare_request(
    server_domain: &str,
    uri: &str,
    headers: &HashMap<String, String>,
    body: impl AsRef<[u8]>,
) -> Result<Request<Full<Bytes>>, hyper::http::Error> {
    let mut request_builder = Request::builder()
        .uri(uri)
        .header("Host", server_domain)
        .header("Accept", "*/*")
        .header("Accept-Encoding", "identity")
        .header("Connection", "close")
        .header("User-Agent", USER_AGENT);

    for (k, v) in headers {
        request_builder = request_builder.header(k, v);
    }

    request_builder.body(Full::new(Bytes::from(body.as_ref().to_vec())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_correctly_prepare_request_body() {
        let request = prepare_request(
            "lotr-api.online",
            "/auth_header_require?param1=value1&param2=value2",
            &HashMap::from([("Authorization".to_string(), "s3cret_t0ken".to_string())]),
            "abc",
        )
        .unwrap();

        let body = request.body();

        assert_eq!(r#"Full { data: Some(b"abc") }"#, format!("{body:?}"));
    }
}
