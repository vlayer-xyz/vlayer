use std::{collections::HashMap, path::Path};

use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs::{create_dir_all, write};
use web_prover::{generate_web_proof, NotaryConfig, TLSN_VERSION};

const NOTARY_HOST: &str = "127.0.0.1";
const NOTARY_PORT: u16 = 7047;
const SERVER_DOMAIN: &str = "lotr-api.online";
const SERVER_HOST: &str = "127.0.0.1";
const SERVER_PORT: u16 = 3011;

#[derive(Deserialize, Debug)]
struct NotaryInfoResponse {
    version: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    check_notary_version().await?;

    generate_valid_web_proof_local_notary().await?;
    generate_valid_web_proof_remote_notary().await?;

    println!("Generate fixtures script completed successfully.");

    Ok(())
}

async fn generate_valid_web_proof_local_notary() -> Result<(), Box<dyn std::error::Error>> {
    let presentation = generate_web_proof(
        NotaryConfig::new(NOTARY_HOST.into(), NOTARY_PORT, "".into(), false),
        SERVER_DOMAIN,
        SERVER_HOST,
        SERVER_PORT,
        "https://lotr-api.online/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        HashMap::new(),
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

    let mut presentation_json: Value = serde_json::from_str(&presentation)?;

    let data = presentation_json["presentationJson"]["data"]
        .as_str()
        .unwrap();
    let modified_data = &data[..data.len().saturating_sub(1)];
    presentation_json["presentationJson"]["data"] = json!(modified_data);

    let modified_json = serde_json::to_string_pretty(&presentation_json).unwrap();

    write_to_file(
        &format!("../../contracts/vlayer/testdata/{TLSN_VERSION}/web_proof_missing_part.json"),
        &modified_json,
    )
    .await?;

    Ok(())
}

async fn generate_valid_web_proof_remote_notary() -> Result<(), Box<dyn std::error::Error>> {
    let presentation = generate_web_proof(
        NotaryConfig::new("notary.pse.dev".into(), 443, "v0.1.0-alpha.8".into(), true),
        SERVER_DOMAIN,
        SERVER_HOST,
        SERVER_PORT,
        "https://lotr-api.online/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        HashMap::new(),
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

async fn write_to_file(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Writing to file: {path}");
    let path = Path::new(path);

    if let Some(parent_dir) = path.parent() {
        create_dir_all(parent_dir).await?;
    }

    write(path, content).await?;

    Ok(())
}
