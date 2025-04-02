use std::collections::HashMap;

use derive_new::new;
use http_body_util::Empty;
use hyper::{body::Bytes, Request, StatusCode};
use hyper_util::rt::TokioIo;
use notary_client::{Accepted, NotarizationRequest, NotaryClient};
use spansy::Spanned;
use tlsn_common::config::ProtocolConfig;
use tlsn_core::{
    attestation::Attestation, request::RequestConfig, transcript::TranscriptCommitConfig,
    CryptoProvider, Secrets,
};
use tlsn_formats::http::{DefaultHttpCommitter, HttpCommit, HttpTranscript};
use tlsn_prover::{Prover, ProverConfig};
use tokio_util::compat::{FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use tracing::debug;

const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36";
const MAX_SENT_DATA: usize = 1 << 12;
const MAX_RECV_DATA: usize = 1 << 14;

#[derive(Debug, Clone, new)]
pub struct NotaryConfig {
    /// Notary host (domain name or IP)
    pub host: String,
    /// Notary port
    pub port: u16,
    /// Notary API path
    pub path_prefix: String,
    /// Whether to use TLS for notary connection
    pub enable_tls: bool,
}

pub async fn notarize(
    notary_config: NotaryConfig,
    server_domain: &str,
    server_host: &str,
    server_port: u16,
    uri: &str,
    headers: HashMap<String, String>,
) -> Result<(Attestation, Secrets), Box<dyn std::error::Error>> {
    let notary_client = NotaryClient::builder()
        .host(notary_config.host)
        .port(notary_config.port)
        .path_prefix(notary_config.path_prefix)
        .enable_tls(notary_config.enable_tls)
        .build()
        .unwrap();

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
        .expect("Could not connect to notary. Make sure it is running.");

    let prover_config = ProverConfig::builder()
        .server_name(server_domain)
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

    let mut request_builder = Request::builder()
        .uri(uri)
        .header("Host", server_domain)
        .header("Accept", "*/*")
        .header("Accept-Encoding", "identity")
        .header("Connection", "close")
        .header("User-Agent", USER_AGENT);

    for (k, v) in &headers {
        request_builder = request_builder.header(k, v);
    }

    let request = request_builder.body(Empty::<Bytes>::new())?;

    debug!("Starting an MPC TLS connection with the server");

    let response = request_sender.send_request(request).await?;

    debug!("Got a response from the server: {}", response.status());

    assert!(response.status() == StatusCode::OK);

    let prover = prover_task.await??;

    let mut prover = prover.start_notarize();

    let transcript = HttpTranscript::parse(prover.transcript())?;

    let body_content = &transcript.responses[0].body.as_ref().unwrap().content;
    let body = String::from_utf8_lossy(body_content.span().as_bytes());

    match body_content {
        tlsn_formats::http::BodyContent::Json(_json) => {
            let parsed = serde_json::from_str::<serde_json::Value>(&body)?;
            debug!("{}", serde_json::to_string_pretty(&parsed)?);
        }
        tlsn_formats::http::BodyContent::Unknown(_span) => {
            debug!("{}", &body);
        }
        _ => {}
    }

    let mut builder = TranscriptCommitConfig::builder(prover.transcript());

    DefaultHttpCommitter::default().commit_transcript(&mut builder, &transcript)?;

    prover.transcript_commit(builder.build()?);

    let request_config = RequestConfig::default();

    let (attestation, secrets) = Box::pin(prover.finalize(&request_config)).await?;
    Ok((attestation, secrets))
}
