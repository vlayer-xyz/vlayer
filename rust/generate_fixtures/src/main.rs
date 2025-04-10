use std::{collections::HashMap, path::Path};

use serde::Deserialize;
use serde_json::{json, Value};
use tlsn_core::{presentation::Presentation, transcript::Transcript};
use tokio::fs::{create_dir_all, write};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;
use utils::range::RangeSet;
use web_proof::web_proof::{PresentationJSON, WebProof};
use web_prover::{
    generate_web_proof, NotarizeParamsBuilder, NotaryConfig, RedactionConfig, TLSN_VERSION,
    TLSN_VERSION_WITH_V_PREFIX,
};

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

const NOTARY_HOST: &str = "127.0.0.1";
const NOTARY_PORT: u16 = 7047;

const REMOTE_NOTARY_HOST: &str = "notary.pse.dev";
const REMOTE_NOTARY_PORT: u16 = 443;

const SERVER_DOMAIN: &str = "lotr-api.online";
const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u16 = 3011;

#[derive(Deserialize, Debug)]
struct NotaryInfoResponse {
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .init();

    check_notary_version().await?;

    generate_valid_web_proof_local_notary().await?;
    generate_valid_web_proof_remote_notary().await?;
    generate_web_proofs_with_redaction().await?;

    info!("Generate fixtures script completed successfully.");

    Ok(())
}

async fn generate_valid_web_proof_local_notary() -> Result<(), Box<dyn std::error::Error>> {
    info!("Generate web proof using local notary");
    let presentation = generate_web_proof(
        NotarizeParamsBuilder::default()
            .notary_config(NotaryConfig::new(NOTARY_HOST.into(), NOTARY_PORT, "".into(), false))
            .server_domain(SERVER_DOMAIN)
            .server_host(SERVER_HOST)
            .server_port(SERVER_PORT)
            .uri("/regular_json?are_you_sure=yes&auth=s3cret_t0ken")
            .build()
            .unwrap(),
    )
    .await?;

    let web_proof = to_web_proof(&presentation)?;

    write_to_file(&format!("../web_proof/testdata/{TLSN_VERSION}/web_proof.json"), &web_proof)
        .await?;

    write_to_file(
        &format!("../../contracts/vlayer/testdata/{TLSN_VERSION}/web_proof.json"),
        &web_proof,
    )
    .await?;

    write_to_file(
        &format!("../../examples/simple-web-proof/testdata/{TLSN_VERSION}/web_proof.json"),
        &web_proof,
    )
    .await?;

    let presentation_json_with_corrupted_data = corrupt_data(&presentation)?;

    write_to_file(
        &format!("../../contracts/vlayer/testdata/{TLSN_VERSION}/web_proof_missing_part.json"),
        &presentation_json_with_corrupted_data,
    )
    .await?;

    write_to_file(
        &format!("../web_proof/testdata/{TLSN_VERSION}/web_proof_identity_name_changed.json"),
        &modify(&web_proof, |value| value["identity"]["name"] = "example.com".into()),
    )
    .await
    .unwrap();

    write_to_file(
        &format!("../web_proof/testdata/{TLSN_VERSION}/web_proof_bad_signature.json"),
        &modify(&web_proof, |value| value["attestation"]["signature"]["data"] = json!([])),
    )
    .await
    .unwrap();

    Ok(())
}

async fn generate_valid_web_proof_remote_notary() -> Result<(), Box<dyn std::error::Error>> {
    info!("Generate web proof using remote notary");
    let presentation = generate_web_proof(
        NotarizeParamsBuilder::default()
            .notary_config(NotaryConfig::new(
                REMOTE_NOTARY_HOST.into(),
                REMOTE_NOTARY_PORT,
                TLSN_VERSION_WITH_V_PREFIX.into(),
                true,
            ))
            .server_domain(SERVER_DOMAIN)
            .server_host(SERVER_HOST)
            .server_port(SERVER_PORT)
            .uri("https://lotr-api.online/regular_json?are_you_sure=yes&auth=s3cret_t0ken")
            .build()
            .unwrap(),
    )
    .await?;

    let web_proof = to_web_proof(&presentation)?;

    write_to_file(
        &format!(
            "../../contracts/vlayer/testdata/{TLSN_VERSION}/web_proof_invalid_notary_pub_key.json"
        ),
        &web_proof,
    )
    .await?;

    write_to_file(
        &format!(
            "../../examples/simple-web-proof/testdata/{TLSN_VERSION}/web_proof_invalid_notary_pub_key.json"
        ),
        &web_proof,
    )
    .await?;

    Ok(())
}

async fn generate_web_proofs_with_redaction() -> Result<(), Box<dyn std::error::Error>> {
    info!("Generate web proofs with redaction");

    generate_web_proofs_with_redaction_config(
        |transcript| RedactionConfig {
            sent: RangeSet::from([0..55, 61..161, 166..transcript.sent().len()]),
            recv: RangeSet::from([0..386, 415..463, 475..transcript.received().len()]),
        },
        &format!("../web_proof/testdata/{TLSN_VERSION}/web_proof_all_redaction_types.json"),
    )
    .await?;

    generate_web_proofs_with_redaction_config(
        |transcript| RedactionConfig {
            sent: RangeSet::from([0..56, 61..161, 166..transcript.sent().len()]),
            recv: RangeSet::from([0..386, 415..463, 475..transcript.received().len()]),
        },
        &format!(
            "../web_proof/testdata/{TLSN_VERSION}/web_proof_request_url_partial_redaction.json"
        ),
    )
    .await?;

    generate_web_proofs_with_redaction_config(
        |transcript| RedactionConfig {
            sent: RangeSet::from([0..55, 61..162, 166..transcript.sent().len()]),
            recv: RangeSet::from([0..386, 415..463, 475..transcript.received().len()]),
        },
        &format!(
            "../web_proof/testdata/{TLSN_VERSION}/web_proof_request_header_partial_redaction.json"
        ),
    )
    .await?;

    generate_web_proofs_with_redaction_config(
        |transcript| RedactionConfig {
            sent: RangeSet::from([0..55, 61..161, 166..transcript.sent().len()]),
            recv: RangeSet::from([0..386, 414..463, 475..transcript.received().len()]),
        },
        &format!(
            "../web_proof/testdata/{TLSN_VERSION}/web_proof_response_header_partial_redaction.json"
        ),
    )
    .await?;

    generate_web_proofs_with_redaction_config(
        |transcript| RedactionConfig {
            sent: RangeSet::from([0..55, 61..161, 166..transcript.sent().len()]),
            recv: RangeSet::from([0..386, 415..464, 475..transcript.received().len()]),
        },
        &format!(
            "../web_proof/testdata/{TLSN_VERSION}/web_proof_response_json_partial_redaction.json"
        ),
    )
    .await?;

    Ok(())
}

async fn generate_web_proofs_with_redaction_config(
    redaction_config: impl Fn(&Transcript) -> RedactionConfig + Send + Sync + 'static,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let presentation = generate_web_proof(
        NotarizeParamsBuilder::default()
            .notary_config(NotaryConfig::new(NOTARY_HOST.into(), NOTARY_PORT, "".into(), false))
            .server_domain(SERVER_DOMAIN)
            .server_host(SERVER_HOST)
            .server_port(SERVER_PORT)
            .uri("https://lotr-api.online/auth_header_require?param1=value1&param2=value2")
            .headers(HashMap::from([("Authorization".to_string(), "s3cret_t0ken".to_string())]))
            .redaction_config_fn(redaction_config)
            .build()
            .unwrap(),
    )
    .await?;

    let web_proof = to_web_proof(&presentation)?;

    write_to_file(path, &web_proof).await?;

    Ok(())
}

async fn check_notary_version() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(format!(r#"http://{NOTARY_HOST}:{NOTARY_PORT}/info"#))
        .await?
        .json::<NotaryInfoResponse>()
        .await?;

    assert_eq!(response.version, TLSN_VERSION, "Notary version mismatch");

    Ok(())
}

fn to_web_proof(presentation: &str) -> Result<String, Box<dyn std::error::Error>> {
    let presentation_json: Value = serde_json::from_str(presentation)?;
    let web_proof = serde_json::to_string_pretty(&presentation_json)? + "\n";

    Ok(web_proof)
}

fn corrupt_data(presentation: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut presentation_json: Value = serde_json::from_str(presentation)?;

    let data = presentation_json["presentationJson"]["data"]
        .as_str()
        .ok_or("Missing or invalid field: 'presentationJson.data'")?;

    let modified_data = &data[..data.len().saturating_sub(1)];
    presentation_json["presentationJson"]["data"] = json!(modified_data);

    Ok(serde_json::to_string_pretty(&presentation_json)?)
}

fn modify<Modifier>(web_proof: &str, modifier: Modifier) -> String
where
    Modifier: Fn(&mut Value),
{
    let web_proof: WebProof = serde_json::from_str(web_proof).unwrap();
    let res: Presentation =
        bincode::deserialize(&hex::decode(web_proof.presentation_json.data).unwrap()).unwrap();

    let presentation_json = serde_json::to_string(&res).unwrap();

    let mut presentation_json: Value = serde_json::from_str(&presentation_json).unwrap();

    modifier(&mut presentation_json);

    let presentation_json: Presentation =
        serde_json::from_str(&serde_json::to_string(&presentation_json).unwrap()).unwrap();

    let data = hex::encode(bincode::serialize(&presentation_json).unwrap());

    let modified_web_proof = WebProof {
        presentation_json: PresentationJSON {
            data,
            ..web_proof.presentation_json
        },
    };

    serde_json::to_string_pretty(&modified_web_proof).unwrap()
}

async fn write_to_file(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(PROJECT_DIR).join(path);
    info!("Writing to file: {}", path.display());

    if let Some(parent_dir) = path.parent() {
        create_dir_all(parent_dir).await?;
    }

    write(path, content).await?;

    Ok(())
}
