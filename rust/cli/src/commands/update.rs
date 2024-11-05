use std::{
    path::{Path, PathBuf},
    process::ExitStatus,
};

use colored::Colorize;
use serde_json::Value;

use super::init::{add_remappings, SoldeerDep};
use crate::errors::CLIError;

pub fn run_update() -> Result<(), CLIError> {
    check_if_vlayerup_exists()?;
    update_cli()?;
    update_sdk()?;
    update_soldeer()?;

    println!("🎉 Update complete.");
    println!("{}", "Build your contracts now and have fun!".bold());

    Ok(())
}

fn check_if_vlayerup_exists() -> Result<(), CLIError> {
    let output = std::process::Command::new("which")
        .arg("vlayerup")
        .output()
        .map_err(into_update_err)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(CLIError::UpgradeError(format!(
            "{} not found. Visit https://book.vlayer.xyz/getting-started/installation.html#get-vlayerup for installation instructions.",
            "vlayerup".italic().bold()
        )))
    }
}

fn update_cli() -> Result<(), CLIError> {
    print_update_intention("vlayer CLI");
    let status = spawn("vlayerup", &["update"])?;
    ensure_success(status, "vlayer CLI")
}

fn update_sdk() -> Result<(), CLIError> {
    if let Some((path, json)) = find_package_json()? {
        do_update_sdk(&path, &json)
    } else {
        warn(&format!("{} not found. Skipping SDK update.", "package.json".bold()))
    }
}

fn do_update_sdk(path: &Path, package_json: &Value) -> Result<(), CLIError> {
    if package_json["dependencies"]["@vlayer/sdk"].is_null() {
        return warn(&format!("{} not found in {}", "@vlayer/sdk".bold(), "package.json".bold()));
    }
    print_update_intention("@vlayer/sdk");
    package_manager(path).update_vlayer()
}

fn update_soldeer() -> Result<(), CLIError> {
    let foundry_toml = find_file_up_tree("foundry.toml")?;
    if let Some(mut foundry_toml_path) = foundry_toml {
        foundry_toml_path.pop();
        do_update_soldeer(&foundry_toml_path)
    } else {
        warn(&format!("{} not found. Skipping Soldeer update.", "foundry.toml".bold()))
    }
}

fn do_update_soldeer(foundry_toml_path: &Path) -> Result<(), CLIError> {
    let version = newest_vlayer_version()?;

    print_update_intention(&format!("vlayer contracts into {}", &version));

    let updated_dep = SoldeerDep {
        name: String::from("vlayer"),
        version,
        url: None,
        remapping: Some(("vlayer-0.1.0", "src").into()),
    };

    updated_dep.install(foundry_toml_path)?;
    add_remappings(foundry_toml_path, &[updated_dep])?;

    Ok(())
}

fn newest_vlayer_version() -> Result<String, CLIError> {
    let output = std::process::Command::new("vlayer")
        .arg("--version")
        .output()
        .map_err(into_update_err)?;

    if !output.status.success() {
        return Err(CLIError::UpgradeError(format!(
            "Failed to run newest vlayer: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    String::from_utf8_lossy(&output.stdout)
        .split_ascii_whitespace()
        .nth(1)
        .map(String::from)
        .ok_or(CLIError::UpgradeError("Corrupted vlayer binary".to_string()))
}

enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    const fn command_args(&self) -> (&str, &str) {
        match self {
            PackageManager::Npm => ("npm", "install"),
            PackageManager::Yarn => ("yarn", "add"),
            PackageManager::Pnpm => ("pnpm", "add"),
            PackageManager::Bun => ("bun", "add"),
        }
    }

    fn install_package(pm_name: &str, install_command: &str) -> Result<(), CLIError> {
        let exit_status = spawn(pm_name, &[install_command, "@vlayer/sdk"])?;
        ensure_success(exit_status, "@vlayer/sdk")
    }

    pub fn update_vlayer(&self) -> Result<(), CLIError> {
        let (pm_name, install_command) = self.command_args();
        Self::install_package(pm_name, install_command)
    }
}

fn package_manager(package_path: &Path) -> PackageManager {
    if package_path.join("bun.lockb").exists() {
        PackageManager::Bun
    } else if package_path.join("pnpm-lock.yaml").exists() {
        PackageManager::Pnpm
    } else if package_path.join("yarn.lock").exists() {
        PackageManager::Yarn
    } else {
        PackageManager::Npm
    }
}

fn find_file_up_tree(name: &str) -> Result<Option<PathBuf>, CLIError> {
    let mut path = std::env::current_dir().map_err(|e| CLIError::UpgradeError(e.to_string()))?;
    loop {
        path.push(name);
        if path.exists() {
            return Ok(Some(path));
        }
        path.pop();
        if !path.pop() {
            return Ok(None);
        }
    }
}

fn find_package_json() -> Result<Option<(PathBuf, Value)>, CLIError> {
    if let Some(mut path) = find_file_up_tree("package.json")? {
        let value =
            serde_json::from_str(&std::fs::read_to_string(&path).map_err(into_update_err)?)?;
        path.pop();
        Ok(Some((path, value)))
    } else {
        Ok(None)
    }
}

fn spawn(command: &str, args: &[&str]) -> Result<ExitStatus, CLIError> {
    std::process::Command::new(command)
        .args(args)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(into_update_err)
}

fn ensure_success(exist_status: ExitStatus, package_name: &str) -> Result<(), CLIError> {
    if exist_status.success() {
        print_successful_update(package_name)
    } else {
        Err(CLIError::UpgradeError(format!("Failed to update {package_name}")))
    }
}

fn warn(message: &str) -> Result<(), CLIError> {
    println!("{} {}\n", "⚠".yellow().bold(), message);
    Ok(())
}

fn print_update_intention(package_name: &str) {
    println!("📦 Updating {}\n", package_name.bold());
}

fn print_successful_update(package_name: &str) -> Result<(), CLIError> {
    println!(
        "{} {} updated {}\n",
        "✔".green().bold(),
        package_name.bold(),
        "successfully".green()
    );
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn into_update_err(e: std::io::Error) -> CLIError {
    CLIError::UpgradeError(e.to_string())
}
