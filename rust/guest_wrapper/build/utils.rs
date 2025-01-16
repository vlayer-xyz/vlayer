use std::{env, fs, io, path::PathBuf};

use risc0_build::GuestListEntry;

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

#[allow(clippy::needless_pass_by_value)]
fn parse_bool_str(s: String) -> anyhow::Result<bool> {
    match s.to_ascii_lowercase().as_str() {
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

/// Remove `GuestListEntry` with given name from vector.
pub fn remove_guest(
    guests: &mut Vec<GuestListEntry>,
    name: &str,
) -> anyhow::Result<GuestListEntry> {
    let idx = guests
        .iter()
        .position(|g| g.name == name)
        .ok_or_else(|| anyhow::anyhow!("Guest {name} not found"))?;
    Ok(guests.remove(idx))
}
