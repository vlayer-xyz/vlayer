use std::process::ExitStatus;

use derive_more::derive::{Deref, From};
use derive_new::new;
use thiserror::Error;

#[derive(From, Deref, new)]
struct Output(std::process::Output);

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to spawn command: {0} {1}")]
    Spawn(std::io::Error, String),
    #[error("Command returned non-zero exit code: {0} {1}")]
    NonZeroExitCode(ExitStatus, String),
}

pub struct Cli;

impl Cli {
    pub fn run(cmd: &str, args: &[&str]) -> Result<String, Error> {
        let output = std::process::Command::new(cmd)
            .args(args)
            .output()
            .map_err(|err| Error::Spawn(err, format!("{cmd} {}", args.join(" "))))?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
            return Err(Error::NonZeroExitCode(output.status, stderr));
        }
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        Ok(stdout)
    }
}
