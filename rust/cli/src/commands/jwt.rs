use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::{Args as ClapArgs, Parser, Subcommand};
use jwt::{
    Algorithm, Claims, ClaimsBuilder, ClaimsBuilderError, DecodingKey, EncodingKey, Environment,
    Error as JwtError, Header, TokenData, Validation, decode, decode_header, encode,
    get_current_timestamp,
};
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("Invalid host string '{0}', expected format <hostname:port>")]
    InvalidHost(String),
    #[error("Specified port value '{0}' not u16")]
    InvalidPort(String),
    #[error("Hostname cannot be empty")]
    EmptyHostname,
    #[error("JWT encoding/decoding error: {0}")]
    Jwt(#[from] JwtError),
    #[error("Invalid Claims args: {0}")]
    ClaimsBuilder(#[from] ClaimsBuilderError),
}

type Result<T> = std::result::Result<T, Error>;

/// Generate and validate Json Web Tokens compatible with vlayer services
/// for local testing
#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Encode a new JWT token
    Encode(Encode),

    /// Decode a JWT token
    Decode(Decode),
}

#[derive(Debug, ClapArgs)]
struct Encode {
    /// Path to private key used for signing JWT
    #[arg(short, long)]
    private_key: PathBuf,

    /// Host url for the Web Proof
    #[arg(long)]
    web_proof_host: Option<String>,

    /// Invalid after N seconds
    #[arg(long)]
    invalid_after: Option<u64>,

    /// Subject
    #[arg(long, default_value_t = String::from("test"))]
    subject: String,

    /// Environment: either test or mainnet
    #[arg(long, default_value_t = Environment::default())]
    environment: Environment,
}

#[derive(Debug, ClapArgs)]
struct Decode {
    /// Path to public key used for verifying JWT signature
    /// If unspecified, will not validate the signature
    #[arg(short, long)]
    public_key: Option<PathBuf>,

    /// JWT to decode
    jwt: String,
}

pub fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Encode(enc) => encode_jwt(enc)?,
        Command::Decode(dec) => decode_jwt(dec)?,
    };
    Ok(())
}

fn encode_jwt(args: Encode) -> Result<()> {
    let priv_key = fs::read(&args.private_key)
        .with_context(|| format!("private key {} not found", args.private_key.display()))?;
    let priv_key = EncodingKey::from_rsa_pem(&priv_key)?;

    let exp = args
        .invalid_after
        .map_or(u64::MAX, |x| get_current_timestamp() + x);

    let mut claims_builder = ClaimsBuilder::default()
        .exp(exp)
        .sub(args.subject)
        .environment(Some(args.environment));

    if let Some(host) = &args.web_proof_host {
        let (host, port) = parse_host(host)?;
        claims_builder = claims_builder.host(host).port(port);
    }

    let claims = claims_builder.build().map_err(Error::ClaimsBuilder)?;

    info!("{claims:#?}");

    let header = Header {
        alg: Algorithm::RS256,
        ..Default::default()
    };

    let jwt = encode(&header, &claims, &priv_key)?;

    info!("{jwt}");

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn decode_jwt(args: Decode) -> Result<()> {
    let pub_key = args
        .public_key
        .as_ref()
        .map(parse_decoding_key)
        .transpose()?;

    let header = decode_header(&args.jwt)?;

    info!("{header:#?}");

    let mut validation = Validation::new(Algorithm::RS256);
    let pub_key = pub_key.unwrap_or_else(|| {
        validation.insecure_disable_signature_validation();
        DecodingKey::from_secret(b"")
    });
    let claims: TokenData<Claims> = decode(&args.jwt, &pub_key, &validation)?;

    info!("{:#?}", claims.claims);

    Ok(())
}

fn parse_decoding_key(path: impl AsRef<Path>) -> Result<DecodingKey> {
    let pub_key = fs::read(path.as_ref())
        .with_context(|| format!("public key {} not found", path.as_ref().display()))?;
    DecodingKey::from_rsa_pem(&pub_key).map_err(Error::Jwt)
}

fn parse_host(host: impl AsRef<str>) -> Result<(String, u16)> {
    let parts: Vec<&str> = host.as_ref().split(':').collect();
    if parts.len() != 2 {
        return Err(Error::InvalidHost(host.as_ref().into()));
    }

    let host_name = parts[0];
    if host_name.is_empty() {
        return Err(Error::EmptyHostname);
    }

    let raw_port = parts[1];
    let port = raw_port
        .parse::<u16>()
        .map_err(|_| Error::InvalidPort(raw_port.into()))?;

    Ok((host_name.to_string(), port))
}

#[cfg(test)]
mod tests {
    use super::{parse_host, *};

    #[test]
    fn parse_host_empty_host() {
        assert!(matches!(parse_host(":").unwrap_err(), Error::EmptyHostname));
        assert!(matches!(parse_host(":443").unwrap_err(), Error::EmptyHostname));
    }

    #[test]
    fn parse_host_invalid_port() {
        assert!(matches!(parse_host("api.x.com:bla").unwrap_err(), Error::InvalidPort { .. }));
        assert!(matches!(parse_host("api.x.com:65536").unwrap_err(), Error::InvalidPort { .. }));
    }

    #[test]
    fn parse_host_invalid_host() {
        let assert_invalid = |host: &str| {
            assert!(matches!(parse_host(host).unwrap_err(), Error::InvalidHost { .. }))
        };
        assert_invalid("");
        assert_invalid("::");
        assert_invalid("443");
        assert_invalid("blablabla");
        assert_invalid("api.x.com");
        assert_invalid("api.x.com:442:443");
    }

    #[test]
    fn parse_host_valid_host() {
        let assert_valid = |host: &str| assert!(parse_host(host).is_ok());
        assert_valid("api.x.com:443");
        assert_valid("api.x.com:8080");
        assert_valid("x.com:112");
        assert_valid("vlayer.xyz:443");
        assert_valid("vlayer.xyz:65535");
    }
}
