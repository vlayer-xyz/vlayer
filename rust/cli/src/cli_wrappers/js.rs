use std::path::Path;

use colored::Colorize;
use derive_new::new;
use thiserror::Error;

use super::base;

pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
    Bun,
}

impl PackageManager {
    pub fn guess(package_path: &Path) -> Option<Self> {
        if package_path.join("bun.lockb").exists() || package_path.join("bun.lock").exists() {
            Some(PackageManager::Bun)
        } else if package_path.join("pnpm-lock.yaml").exists() {
            Some(PackageManager::Pnpm)
        } else if package_path.join("yarn.lock").exists() {
            Some(PackageManager::Yarn)
        } else if package_path.join("package-lock.json").exists() {
            Some(PackageManager::Npm)
        } else {
            None
        }
    }

    const fn name(&self) -> &str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
            PackageManager::Bun => "bun",
        }
    }

    const fn install_command(&self) -> &str {
        match self {
            PackageManager::Npm => "install",
            PackageManager::Yarn | PackageManager::Pnpm | PackageManager::Bun => "add",
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Cli(#[from] base::Error),
}

#[derive(new)]
pub struct Cli(PackageManager);

impl Cli {
    pub fn install(&self, name: &str, version: &str) -> Result<(), Error> {
        base::Cli::run(
            self.0.name(),
            &[self.0.install_command(), format!("{name}@{version}").as_str()],
        )?;
        println!("{} {} updated {}\n", "✔".green().bold(), name.bold(), "successfully".green());
        Ok(())
    }
}
