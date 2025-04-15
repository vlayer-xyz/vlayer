use thiserror::Error;

use super::base;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Cli(#[from] base::Error),
    #[error("Invalid version format: {0}")]
    InvalidVersionFormat(String),
}

pub struct Cli;

impl Cli {
    pub fn version() -> Result<String, Error> {
        Cli::parse_version(base::Cli::run("vlayer", &["--version"])?)
    }

    fn parse_version(output: String) -> Result<String, Error> {
        output
            .split_ascii_whitespace()
            .nth(1)
            .map(String::from)
            .ok_or(Error::InvalidVersionFormat(output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_version_string() -> Result<(), Error> {
        assert_eq!(
            Cli::parse_version("vlayer 0.1.0-nightly-20250402-7a7dcdf".to_string())?,
            "0.1.0-nightly-20250402-7a7dcdf"
        );
        Ok(())
    }
}
