use std::process::ExitStatus;

use derive_more::derive::{Deref, From};
use derive_new::new;

#[derive(From, Deref, new)]
struct Output(std::process::Output);

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to spawn command: {0}")]
    Spawn(#[from] std::io::Error),
    #[error("Command returned non-zero exit code: {0}")]
    NonZeroExitCode(ExitStatus, String),
}

pub struct Cli;

impl Cli {
    pub fn run(cmd: &str, args: &[&str]) -> Result<String, Error> {
        let output = std::process::Command::new(cmd).args(args).output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            return Err(Error::NonZeroExitCode(output.status, stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(stdout)
    }
}
