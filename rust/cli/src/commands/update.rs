use std::path::PathBuf;

use colored::Colorize;
use serde_json::Value;

use crate::errors::CLIError;

pub async fn run_update() -> Result<(), CLIError> {
    check_if_vlayerup_exists()?;
    update_cli()?;
    update_sdk()?;
    update_soldeer()?;
    Ok(())
}

fn check_if_vlayerup_exists() -> Result<(), CLIError> {
    let output = std::process::Command::new("which")
        .arg("vlayerup")
        .output()
        .map_err(|e| CLIError::UpgradeError(e.to_string()))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(CLIError::UpgradeError(format!(
            "{} not found. Visit https://book.vlayer.xyz/getting-started/installation.html#get-vlayerup and install it.",
            "vlayerup".italic().bold()
        )))
    }
}

fn update_cli() -> Result<(), CLIError> {
    print_update_intention("vlayer CLI");

    let status = std::process::Command::new("vlayerup")
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|e| CLIError::UpgradeError(e.to_string()))?;

    if status.success() {
        print_successful_update("vlayer CLI");
        Ok(())
    } else {
        Err(CLIError::UpgradeError("Failed to update vlayerup".into()))
    }
}

fn update_sdk() -> Result<(), CLIError> {
    if let Some((path, json)) = find_package_json()? {
        do_update_sdk(path, json)
    } else {
        println!("{} package.json not found. Skipping SDK update.", "⚠".yellow().bold());
        Ok(())
    }
}

fn do_update_sdk(path: PathBuf, package_json: Value) -> Result<(), CLIError> {
    if package_json["dependencies"]["@vlayer/sdk"].is_null() {
        println!("{} @vlayer/sdk not found in package.json", "⚠".yellow().bold());
        return Ok(());
    }
    print_update_intention("@vlayer/sdk");
    select_package_manager(path).update_vlayer()
}

fn update_soldeer() -> Result<(), CLIError> {
    let foundry_toml = find_file_up_tree("foundry.toml")?;
    if foundry_toml.is_none() {
        println!("{} foundry.toml not found. Skipping Soldeer update.", "⚠".yellow().bold());
        return Ok(());
    }
    let mut foundry_toml_path = foundry_toml.unwrap();
    foundry_toml_path.pop();
    print_update_intention("vlayer contracts");
    let status = std::process::Command::new("forge")
        .arg("soldeer")
        .arg("install")
        .arg("vlayer~latest")
        .current_dir(foundry_toml_path)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(|e| CLIError::UpgradeError(e.to_string()))?;

    if status.success() {
        print_successful_update("vlayer contracts");
        Ok(())
    } else {
        Err(CLIError::UpgradeError("Failed to update Soldeer".into()))
    }
}

enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    fn command_args(&self) -> (&str, &str) {
        match self {
            PackageManager::Npm => ("npm", "install"),
            PackageManager::Yarn => ("yarn", "add"),
            PackageManager::Pnpm => ("pnpm", "add"),
            PackageManager::Bun => ("bun", "add"),
        }
    }

    fn spawn_install(arg1: &str, arg2: &str) -> Result<(), CLIError> {
        let exit_status = std::process::Command::new(arg1)
            .arg(arg2)
            .arg("@vlayer/sdk")
            .spawn()?
            .wait()?;
        if exit_status.success() {
            print_successful_update("@vlayer/sdk");
            Ok(())
        } else {
            Err(CLIError::UpgradeError("Failed to update @vlayer/sdk".into()))
        }
    }

    pub fn update_vlayer(&self) -> Result<(), CLIError> {
        let (arg1, arg2) = self.command_args();
        Self::spawn_install(arg1, arg2)
    }
}

fn select_package_manager(package_path: PathBuf) -> PackageManager {
    if package_path.join("bun.lockb").exists() {
        PackageManager::Bun
    } else if package_path.join("pnpm-lock.yaml").exists() {
        PackageManager::Yarn
    } else if package_path.join("yarn.lock").exists() {
        PackageManager::Pnpm
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
        let value = serde_json::from_str(
            &std::fs::read_to_string(&path).map_err(|e| CLIError::UpgradeError(e.to_string()))?,
        )?;
        path.pop();
        Ok(Some((path, value)))
    } else {
        Ok(None)
    }
}

fn print_update_intention(package_name: &str) {
    println!("📦 Updating {}", package_name.bold());
}

fn print_successful_update(package_name: &str) {
    println!(
        "{} {} updated {}\n",
        "✔".green().bold(),
        package_name.bold(),
        "successfully".green()
    );
}
