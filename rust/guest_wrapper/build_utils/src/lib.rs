pub mod chain_guest_id;
mod data_layout;

#[cfg(feature = "risc0")]
mod risc0_builder;

use std::{env, fs, io, path::PathBuf};

#[cfg(feature = "risc0")]
pub use risc0_builder::Builder as Risc0Builder;
use risc0_zkp::core::digest::Digest;

const PROJECT_ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../..");

pub fn use_var(key: &str) -> Option<String> {
    println!("cargo:rerun-if-env-changed={key}");
    env::var(key).ok()
}

pub fn path_from_env(key: &str) -> Option<PathBuf> {
    env::var(key).map(Into::into).ok()
}

/// Parse boolean env variable:
///   * true if variable is one of "1", "true", "yes",
///   * false if variables is one of "0", "false", "no",
///   * false if variable is unset,
///   * panic if variable is set to unexpected value.
pub fn use_bool_var(key: &str) -> bool {
    use_var(key)
        .map(parse_bool_str)
        .transpose()
        .unwrap_or_else(|_| panic!("Invalid value for {key}"))
        .unwrap_or(false)
}

pub fn parse_bool_str(s: impl AsRef<str>) -> anyhow::Result<bool> {
    match s.as_ref().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" => Ok(true),
        "0" | "false" | "no" => Ok(false),
        s => anyhow::bail!("Invalid value: '{s}' accepted: 1/0/true/false/yes/no"),
    }
}

pub fn remove_file_if_exists(path: &PathBuf) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        res => res,
    }
}

pub fn decode_hex_id(hex_id: impl AsRef<[u8]>) -> anyhow::Result<Digest> {
    let mut bytes = [0_u8; 32];
    hex::decode_to_slice(hex_id, &mut bytes)?;
    Ok(Digest::from_bytes(bytes))
}
