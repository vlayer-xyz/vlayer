use std::process::Output;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to spawn command: {0}")]
    Spawn(#[from] std::io::Error),
    #[error("Failed to run vlayer --version: {0}")]
    Version(String),
    #[error("Invalid version format: {0}")]
    InvalidVersionFormat(String),
}

pub struct Cli;

impl Cli {
    pub fn version() -> Result<String, Error> {
        Cli::parse_version(Cli::run_version()?)
    }

    fn run_version() -> Result<String, Error> {
        let Output {
            status,
            stdout,
            stderr,
        } = std::process::Command::new("vlayer")
            .arg("--version")
            .output()?;

        if !status.success() {
            return Err(Error::Version(String::from_utf8_lossy(&stderr).into_owned()));
        }

        Ok(String::from_utf8_lossy(&stdout).into_owned())
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
