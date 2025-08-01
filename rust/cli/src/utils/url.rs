use std::{
    fs,
    path::{Path, PathBuf},
};

use regex::Regex;
use tracing::info;
use version::is_stable;

use crate::{errors::Error, version};

pub fn update_prover_url(root_path: &Path) -> Result<(), Error> {
    let channel = if is_stable() { "stable" } else { "nightly" };
    let version = is_stable().then(version);

    fn update_file(
        path: PathBuf,
        label: &str,
        channel: &str,
        version: Option<&str>,
    ) -> Result<(), Error> {
        let content = fs::read_to_string(&path)?;
        info!("Updating prover URL in {}", label);
        let output = modify_url_with_channel_and_version(&content, channel, version)?;
        fs::write(path, output)?;
        Ok(())
    }

    for rel in &["vlayer/.env.testnet", "vlayer/.env.mainnet"] {
        let path = root_path.join(rel);
        update_file(path, rel, channel, version.as_deref())?;
    }

    Ok(())
}

pub fn modify_url_with_channel_and_version(
    file_content: &str,
    channel: &str,
    version: Option<&str>,
) -> Result<String, regex::Error> {
    // Match URLs with or without existing version paths
    let re = Regex::new(r"https://(stable|nightly|dev)-([^.]+)\.vlayer\.xyz(?:/[^\s]*)?/?")?;

    let replacement = match version {
        Some(v) => format!("https://{channel}-$2.vlayer.xyz/{v}/"),
        None => format!("https://{channel}-$2.vlayer.xyz"),
    };

    Ok(re.replace_all(file_content, replacement).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_with_version() {
        let content = "PROVER_URL=https://stable-fake-prover.vlayer.xyz";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("1.3.0")).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://stable-fake-prover.vlayer.xyz/1.3.0/");
    }

    #[test]
    fn nightly_without_version() {
        let content = "PROVER_URL=https://nightly-fake-prover.vlayer.xyz";
        let modified_url = modify_url_with_channel_and_version(content, "nightly", None).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://nightly-fake-prover.vlayer.xyz");
    }

    #[test]
    fn stable_url_with_existing_version() {
        let content = "PROVER_URL=https://stable-fake-prover.vlayer.xyz/0.9.0/";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("1.3.0")).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://stable-fake-prover.vlayer.xyz/1.3.0/");
    }

    #[test]
    fn change_channel_stable_to_nightly_remove_version() {
        let content = "PROVER_URL=https://stable-fake-prover.vlayer.xyz/1.2.0/";
        let modified_url = modify_url_with_channel_and_version(content, "nightly", None).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://nightly-fake-prover.vlayer.xyz");
    }

    #[test]
    fn change_channel_nightly_to_stable_add_version() {
        let content = "PROVER_URL=https://nightly-fake-prover.vlayer.xyz";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("2.0.0")).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://stable-fake-prover.vlayer.xyz/2.0.0/");
    }

    #[test]
    fn env_file_with_version() {
        let content = "CHAIN_NAME=optimismSepolia\nPROVER_URL=https://stable-fake-prover.vlayer.xyz\nJSON_RPC_URL=https://sepolia.optimism.io\n";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("1.5.0")).unwrap();
        assert_eq!(
            modified_url,
            "CHAIN_NAME=optimismSepolia\nPROVER_URL=https://stable-fake-prover.vlayer.xyz/1.5.0/\nJSON_RPC_URL=https://sepolia.optimism.io\n"
        );
    }

    #[test]
    fn url_with_trailing_slash() {
        let content = "PROVER_URL=https://stable-fake-prover.vlayer.xyz/";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("1.3.0")).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://stable-fake-prover.vlayer.xyz/1.3.0/");
    }

    #[test]
    fn url_with_complex_existing_path() {
        let content = "PROVER_URL=https://stable-fake-prover.vlayer.xyz/old/version/1.0.0";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("2.1.0")).unwrap();
        assert_eq!(modified_url, "PROVER_URL=https://stable-fake-prover.vlayer.xyz/2.1.0/");
    }

    #[test]
    fn no_url() {
        let content = "some random text";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("1.0.0")).unwrap();
        assert_eq!(modified_url, "some random text");
    }

    #[test]
    fn empty() {
        let content = "";
        let modified_url =
            modify_url_with_channel_and_version(content, "stable", Some("1.0.0")).unwrap();
        assert_eq!(modified_url, "");
    }
}
