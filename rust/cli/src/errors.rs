use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Command execution failed: {0}")]
    CommandExecutionError(#[from] std::io::Error),
    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Git command failed: {0}")]
    GitError(String),
    #[error("No foundry.toml file found")]
    NoFoundryError,
    #[error("No src field found in foundry.toml")]
    NoSrcInFoundryToml,
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error("Project directory not found: {0}")]
    SrcDirNotFound(PathBuf),
    #[error("Error downloading vlayer examples: {0}")]
    DownloadExamplesError(#[from] reqwest::Error),
    #[error("Failed initializing Forge: {0}")]
    ForgeInitError(String),
    #[error("{0}")]
    UpgradeError(String),
    #[error("Error parsing package.json: {0}")]
    PackageJsonError(#[from] serde_json::Error),
    #[error("{0} tests failed")]
    TestsFailed(usize),
    #[error("{0}")]
    TestsExecutionError(#[from] test_runner::Report),
    #[error("TOML deserialization failed: {0}")]
    TomlError(#[from] toml::de::Error),
}

impl Error {
    pub fn error_code(&self) -> i32 {
        match self {
            Error::TestsFailed(failed) => {
                i32::try_from(*failed).expect("Failed tests count is too large")
            }
            _ => 1,
        }
    }
}
