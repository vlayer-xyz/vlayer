use std::path::PathBuf;

use soldeer_core::errors::SoldeerError;
use thiserror::Error;

use crate::{
    cli_wrappers::{base, js, vlayer},
    commands::{jwt::Error as JwtError, update},
    config::Error as ConfigError,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Command execution failed: {0}")]
    CommandExecution(#[from] std::io::Error),
    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Git command failed: {0}")]
    Git(String),
    #[error("No foundry.toml file found")]
    NoFoundry,
    #[error("No src field found in foundry.toml")]
    NoSrcInFoundryToml,
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("Project directory not found: {0}")]
    SrcDirNotFound(PathBuf),
    #[error("Error downloading vlayer examples: {0}")]
    DownloadExamples(#[from] reqwest::Error),
    #[error("Failed initializing Forge: {0}")]
    ForgeInit(String),
    #[error("{0}")]
    Upgrade(String),
    #[error("Error parsing package.json: {0}")]
    PackageJson(#[from] serde_json::Error),
    #[error("{0} tests failed")]
    TestsFailed(usize),
    #[error("{0}")]
    TestsExecution(#[from] test_runner::Report),
    #[error("TOML deserialization failed: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Soldeer failed: {0}")]
    Soldeer(#[from] SoldeerError),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Vlayer(#[from] vlayer::Error),
    #[error(transparent)]
    JsPm(#[from] js::Error),
    #[error(transparent)]
    Cli(#[from] base::Error),
    #[error(transparent)]
    UpdateDocker(#[from] update::docker::Error),
    #[error(transparent)]
    Jwt(#[from] JwtError),
    #[error(transparent)]
    Regex(#[from] regex::Error),
}

impl Error {
    #[allow(clippy::expect_used)]
    pub fn error_code(&self) -> i32 {
        match self {
            Error::TestsFailed(failed) => {
                i32::try_from(*failed).expect("Failed tests count is too large")
            }
            _ => 1,
        }
    }
}
