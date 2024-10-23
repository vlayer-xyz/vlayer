use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CLIError {
    #[error("Command execution failed: {0}")]
    CommandExecutionError(#[from] std::io::Error),
    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Git command failed: {0}")]
    GitError(String),
    #[error("No foundry.toml file found")]
    NoFoundryError,
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("Error parsing TOML:\n {0}")]
    TomlError(String),
    #[error("Project directory not found: {0}")]
    SrcDirNotFound(PathBuf),
    #[error("Error downloading vlayer examples: {0}")]
    DownloadExamplesError(reqwest::Error),
    #[error("Failed initializing Forge: {0}")]
    ForgeInitError(String),
    #[error("Error parsing package.json: {0}")]
    PackageJsonError(#[from] serde_json::Error),
    #[error("{0} tests failed")]
    TestFailed(usize),
}

impl CLIError {
    pub fn error_code(&self) -> i32 {
        match self {
            CLIError::TestFailed(failed) => {
                i32::try_from(*failed).expect("Failed tests count is too large")
            }
            _ => 1,
        }
    }
}
