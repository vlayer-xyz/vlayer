pub use clap::Parser;
use clap::builder::{IntoResettable, Resettable, Str};
use config::{Config as EnvConfig, Environment};
use guest_wrapper::CALL_GUEST_ELF;

use crate::config::{ConfigOptions, ConfigOptionsWithVersion, parse_config_file};

#[derive(Parser)]
#[command(version = Version)]
pub struct Cli {
    /// Path to TOML config file such as config.toml.
    /// See https://book.vlayer.xyz/appendix/architecture/prover.html#toml for options.
    #[arg(long)]
    config_file: Option<String>,
}

impl TryFrom<Cli> for ConfigOptionsWithVersion {
    type Error = anyhow::Error;

    fn try_from(value: Cli) -> Result<Self, Self::Error> {
        let config = match value.config_file {
            Some(path) => parse_config_file(path)?,
            None => config_from_env()?,
        };
        let semver = version::version();
        Ok(ConfigOptionsWithVersion { semver, config })
    }
}

fn config_from_env() -> anyhow::Result<ConfigOptions> {
    let default_config = EnvConfig::try_from(&ConfigOptions::default())?;
    let env_config = Environment::with_prefix("VLAYER")
        .try_parsing(true)
        .prefix_separator("_")
        .separator("__")
        .list_separator(" ")
        .with_list_parse_key("rpc_urls")
        .with_list_parse_key("auth.jwt.claims")
        .ignore_empty(true);
    let config = EnvConfig::builder()
        .add_source(default_config)
        .add_source(env_config)
        .build()?
        .try_deserialize()?;
    Ok(config)
}

struct Version;

impl IntoResettable<Str> for Version {
    fn into_resettable(self) -> Resettable<Str> {
        version_msg().into_resettable()
    }
}

fn version_msg() -> String {
    [version::version(), call_guest_id()].join("\n")
}

fn call_guest_id() -> String {
    let little_endian_hex = hex::encode(CALL_GUEST_ELF.id);
    format!("CALL_GUEST_ID: {little_endian_hex}")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod version_msg {
        use super::*;

        #[test]
        fn contains_version_line() {
            let version_msg = version_msg();
            let second_line = version_msg.lines().next().unwrap();

            assert_eq!(second_line, version::version());
        }

        #[test]
        fn contains_guest_id() {
            let version = version_msg();
            let second_line = version.lines().nth(1).unwrap();

            assert_eq!(second_line, call_guest_id());
        }
    }

    mod guest_id {
        use regex::Regex;

        use super::*;

        #[test]
        fn guest_id_equals_to_compiled_in_version() {
            let guest_id_line = call_guest_id();
            let id_regex = Regex::new(r"^CALL_GUEST_ID: [a-f0-9]{64}$").unwrap();
            assert!(id_regex.is_match(&guest_id_line));
        }
    }
}
