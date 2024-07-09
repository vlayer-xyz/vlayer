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
    #[error("Error downloading vlayer contracts: {0}")]
    DownloadVlayerFilesError(reqwest::Error),
    #[error("Error inserting vlayer contracts: following already exist")]
    // TODO: add list of files to error message
    InsertVlayerFilesError(Vec<String>),
}
