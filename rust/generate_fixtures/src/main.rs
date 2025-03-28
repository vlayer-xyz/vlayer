use std::{collections::HashMap, path::Path};

use serde::Deserialize;
use serde_json::Value;
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

    let web_proof = get_web_proof().await?;

    save_fixture(&web_proof).await?;

    println!("Generate fixtures script completed successfully.");

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

async fn get_web_proof() -> Result<String, Box<dyn std::error::Error>> {
    let presentation = generate_web_proof(
        NotaryConfig::new(NOTARY_HOST.into(), NOTARY_PORT, "".into(), false),
        SERVER_DOMAIN,
        SERVER_HOST,
        SERVER_PORT,
        "/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
        HashMap::new(),
    )
    .await?;

    let presentation_json: Value = serde_json::from_str(&presentation)?;
    let web_proof = serde_json::to_string_pretty(&presentation_json)? + "\n";

    Ok(web_proof)
}

async fn save_fixture(web_proof: &str) -> Result<(), Box<dyn std::error::Error>> {
    write_to_file(&format!("../web_proof/testdata/{TLSN_VERSION}/web_proof.json"), web_proof)
        .await?;

    Ok(())
}

async fn write_to_file(path: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(path);

    if let Some(parent_dir) = path.parent() {
        create_dir_all(parent_dir).await?;
    }

    write(path, content).await?;

    Ok(())
}
