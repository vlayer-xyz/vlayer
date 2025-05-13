use std::{net::SocketAddr, path::PathBuf};

use clap::Parser;
use common::GlobalArgs;
use server_utils::jwt::cli::Args as JwtArgs;

#[derive(Debug, clap::Args)]
#[group(required = false, multiple = false)]
pub(crate) struct PrivateKeyArgs {
    #[arg(
        long,
        short = 'k',
        env = "PRIVATE_KEY",
        help = "Private key in PEM format"
    )]
    private_key: Option<String>,
    #[arg(long, short = 'f', env = "PRIVATE_KEY_PATH", help = "Path to PEM file")]
    private_key_path: Option<PathBuf>,
}

#[derive(Debug, Parser)]
#[command(version = version::version())]
pub(crate) struct Cli {
    #[arg(
        long,
        short,
        env,
        help = "Socket address to listen on",
        default_value = "127.0.0.1:3002"
    )]
    pub(crate) listen_addr: SocketAddr,

    #[clap(flatten)]
    pub(crate) private_key: PrivateKeyArgs,

    #[clap(flatten)]
    pub(crate) jwt_args: JwtArgs,

    #[clap(flatten)]
    pub(crate) global_args: GlobalArgs,
}

impl PrivateKeyArgs {
    pub(crate) fn private_key(self) -> Result<Option<String>, std::io::Error> {
        self.private_key_path
            .map(std::fs::read_to_string)
            .transpose()
            .map(|key| key.or(self.private_key))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const PRIVATE_KEY: &str = include_str!("../test_fixtures/signer.pem");

    #[test]
    fn when_path_to_private_key_provided_returns_file_content() -> anyhow::Result<()> {
        let args = PrivateKeyArgs {
            private_key: Some("ignored param".into()),
            private_key_path: Some("test_fixtures/signer.pem".into()),
        };
        assert_eq!(args.private_key()?.as_deref(), Some(PRIVATE_KEY));
        Ok(())
    }

    #[test]
    fn test_when_only_plain_key_provided_returns_this_key() -> anyhow::Result<()> {
        let args = PrivateKeyArgs {
            private_key: Some(PRIVATE_KEY.into()),
            private_key_path: None,
        };
        assert_eq!(args.private_key()?.as_deref(), Some(PRIVATE_KEY));
        Ok(())
    }

    #[test]
    fn test_when_nothing_is_provided_returns_none() -> anyhow::Result<()> {
        let args = PrivateKeyArgs {
            private_key: None,
            private_key_path: None,
        };
        assert_eq!(args.private_key()?, None);
        Ok(())
    }
}
