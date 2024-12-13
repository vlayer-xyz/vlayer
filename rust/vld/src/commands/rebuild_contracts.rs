use std::{fs::File, path::Path, process::{Command, Stdio}};

use glob::{GlobError, PatternError, glob};

#[derive(thiserror::Error, Debug)]
pub enum ContractRebuildError{
    #[error("Failed to execute command: {0}")]
    IO(#[from] std::io::Error),
    #[error("Faileds to read pattern: {0}")]
    Pattern(#[from] PatternError),
    #[error("Failed to find files: {0}")]
    Glob(#[from] GlobError),
}

pub fn rebuild_contracts(contracts_path: &str) -> Result<(), ContractRebuildError> {
    println!("Rebuilding contracts");

    let fixtures_path = Path::new(contracts_path).join("fixtures");
    Command::new("forge").arg("soldeer").arg("install").current_dir(&fixtures_path).output()?;
    Command::new("forge").arg("clean").current_dir(&fixtures_path).output()?;
    Command::new("forge").arg("build").current_dir(&fixtures_path).output()?;

    let vlayer_path = Path::new(contracts_path).join("vlayer");
    Command::new("forge").arg("soldeer").arg("install").current_dir(&vlayer_path).output()?;
    Command::new("forge").arg("clean").current_dir(&vlayer_path).output()?;
    Command::new("forge").arg("build").current_dir(&vlayer_path).output()?;

    println!("Building ts types");
    for output_json in glob("*/out/*.sol/*.json")? {
        let output_json = output_json?;
        let output_ts = output_json.join(".ts");
        let cat_output = Command::new("cat").arg(output_json).stdout(Stdio::piped()).spawn()?;
        Command::new("jq").stdin(cat_output.stdout.unwrap()).stdout(File::create(output_ts)?);
    }

    Ok(())
}