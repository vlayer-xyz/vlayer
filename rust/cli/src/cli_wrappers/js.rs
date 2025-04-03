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
    pub fn guess(package_path: &Path) -> Self {
        if package_path.join("bun.lockb").exists() || package_path.join("bun.lock").exists() {
            PackageManager::Bun
        } else if package_path.join("pnpm-lock.yaml").exists() {
            PackageManager::Pnpm
        } else if package_path.join("yarn.lock").exists() {
            PackageManager::Yarn
        } else {
            PackageManager::Npm
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
    pub fn install(&self, name: &str) -> Result<(), Error> {
        base::Cli::run(self.0.name(), &[self.0.install_command(), name])?;
        println!("{} {} updated {}\n", "✔".green().bold(), name.bold(), "successfully".green());
        Ok(())
    }
}
