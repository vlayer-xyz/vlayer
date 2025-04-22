use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use docker::update_docker;
use logger::UpdateLogger;
use serde::Deserialize;
use serde_json::Value;
use strum::Display;

use crate::{
    cli_wrappers::{base, js, vlayer},
    config::{Config, DEFAULT_CONFIG},
    errors::{Error, Result},
    soldeer::{add_remappings, install_solidity_dependencies},
    utils::path::find_file_up_tree,
};

pub mod docker;
mod logger;

#[derive(Clone, Copy, Debug, ValueEnum, Default, Display, Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum ReleaseChannel {
    Nightly,
    #[default]
    Stable,
}

#[derive(Clone, Debug, Parser)]
pub(crate) struct UpdateArgs {
    /// Vlayer release channel
    #[arg(
        long,
        value_enum,
        env = "VLAYER_RELEASE_CHANNEL",
        default_value = "stable"
    )]
    channel: ReleaseChannel,
}

pub async fn run_update(args: UpdateArgs) -> Result<()> {
    ensure_vlayerup_exists()?;
    update_cli(args.channel)?;
    update_sdk()?;
    update_contracts().await?;
    update_docker()?;

    println!("🎉 Update complete.");
    println!("{}", "Build your contracts now and have fun!".bold());

    Ok(())
}

fn ensure_vlayerup_exists() -> Result<()> {
    base::Cli::run("which", &["vlayerup"])
        .map(|_| ())
        .map_err(|_| {
            Error::Upgrade(format!(
                "{} not found. Visit https://book.vlayer.xyz/getting-started/installation.html#get-vlayerup for installation instructions.",
                "vlayerup".italic().bold()
            ))
        })
}

fn update_cli(channel: ReleaseChannel) -> Result<()> {
    let logger = UpdateLogger::new("CLI");
    let previous_version = vlayer::Cli::version()?;
    base::Cli::run("vlayerup", &["--channel", channel.to_string().as_str()])?;
    let updated_version = vlayer::Cli::version()?;
    logger.success_with_version_info(&previous_version, &updated_version);
    Ok(())
}

fn update_sdk() -> Result<()> {
    let version = vlayer::Cli::version()?;
    let logger = UpdateLogger::new(format!("SDK to {version}"));
    let Some((path, package_json)) = find_package_json()? else {
        logger.warn(format!("{} not found. Skipping SDK update.", "package.json".bold()));
        return Ok(());
    };

    let Some(js_pm) = js::PackageManager::guess(&path) else {
        return Err(Error::Upgrade("Failed to guess which JS package manager is used".to_string()));
    };
    let js_pm_cli = js::Cli::new(js_pm);

    if !package_json["dependencies"]["@vlayer/sdk"].is_null() {
        js_pm_cli.install("@vlayer/sdk", version.as_str())?;
    }
    if !package_json["dependencies"]["@vlayer/react"].is_null() {
        js_pm_cli.install("@vlayer/react", version.as_str())?;
    }

    logger.success();
    Ok(())
}

#[allow(clippy::unwrap_used)]
async fn update_contracts() -> Result<()> {
    let version = vlayer::Cli::version()?;
    let logger = UpdateLogger::new(format!("Contracts to {}", &version));
    let foundry_toml = find_file_up_tree("foundry.toml")?;
    let Some(foundry_toml_path) = foundry_toml else {
        logger.warn(format!("{} not found. Skipping Soldeer update.", "foundry.toml".bold()));
        return Ok(());
    };
    let foundry_root = foundry_toml_path.parent().unwrap();

    let config = Config::from_str(DEFAULT_CONFIG.replace("{{VERSION}}", version.as_str()))?;
    install_solidity_dependencies(&config.sol_dependencies).await?;
    add_remappings(foundry_root, config.sol_dependencies.values())?;
    logger.success();

    Ok(())
}

fn find_package_json() -> Result<Option<(PathBuf, Value)>> {
    if let Some(mut path) = find_file_up_tree("package.json")? {
        let value =
            serde_json::from_str(&std::fs::read_to_string(&path).map_err(into_update_err)?)?;
        path.pop();
        Ok(Some((path, value)))
    } else {
        Ok(None)
    }
}

#[allow(clippy::needless_pass_by_value)]
fn into_update_err(e: std::io::Error) -> Error {
    Error::Upgrade(e.to_string())
}
