use std::{
    fs::File,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use glob::{glob, GlobError, PatternError};

use crate::config;
#[derive(thiserror::Error, Debug)]
pub enum ContractRebuildError {
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
    Command::new("forge")
        .arg("soldeer")
        .arg("install")
        .current_dir(&fixtures_path)
        .output()?;
    Command::new("forge")
        .arg("clean")
        .current_dir(&fixtures_path)
        .output()?;
    Command::new("forge")
        .arg("build")
        .current_dir(&fixtures_path)
        .output()?;

    let vlayer_path = Path::new(contracts_path).join("vlayer");
    Command::new("forge")
        .arg("soldeer")
        .arg("install")
        .current_dir(&vlayer_path)
        .output()?;
    Command::new("forge")
        .arg("clean")
        .current_dir(&vlayer_path)
        .output()?;
    Command::new("forge")
        .arg("build")
        .current_dir(&vlayer_path)
        .output()?;

    println!("Building ts types");
    let vlayer_home = config::get_vlayer_path();
    let pattern = format!("{vlayer_home}/**/out/**/*.json");
    println!("pattern: {pattern}");
    for output_json in glob(&pattern)? {
        let output_json = output_json?;
        let output_ts = output_json.with_extension("ts");

        println!("Generating {}", output_ts.display());
        let mut ts_file = File::create(&output_ts)?;
        writeln!(ts_file, "export default <const>")?;

        let json_contents = std::fs::read_to_string(&output_json)?;
        let mut output = Command::new("jq")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        let mut stdin = output.stdin.take().unwrap();
        stdin.write_all(json_contents.as_bytes())?;
        drop(stdin);

        let formatted_json = output.wait_with_output()?;
        ts_file.write_all(&formatted_json.stdout)?;
    }

    Ok(())
}
