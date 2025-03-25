use std::{
    fs,
    path::{Path, PathBuf},
    process::ExitStatus,
};

use colored::Colorize;
use serde_json::Value;

use crate::{
    config::{Config, Error as ConfigError, SolDependencies},
    errors::{Error, Result},
    soldeer::{add_remappings, install},
};

pub async fn run_update() -> Result<()> {
    check_if_vlayerup_exists()?;
    update_cli()?;
    update_sdk()?;
    update_contracts().await?;
    update_docker()?;

    println!("🎉 Update complete.");
    println!("{}", "Build your contracts now and have fun!".bold());

    Ok(())
}

fn check_if_vlayerup_exists() -> Result<()> {
    let output = std::process::Command::new("which")
        .arg("vlayerup")
        .output()
        .map_err(into_update_err)?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::Upgrade(format!(
            "{} not found. Visit https://book.vlayer.xyz/getting-started/installation.html#get-vlayerup for installation instructions.",
            "vlayerup".italic().bold()
        )))
    }
}

fn update_cli() -> Result<()> {
    print_update_intention("vlayer CLI");
    let status = spawn("vlayerup", &["update"])?;
    ensure_success(status, "vlayer CLI")
}

fn update_sdk() -> Result<()> {
    if let Some((path, json)) = find_package_json()? {
        do_update_sdk(&path, &json)
    } else {
        warn(&format!("{} not found. Skipping SDK update.", "package.json".bold()))
    }
}

fn do_update_sdk(path: &Path, package_json: &Value) -> Result<()> {
    if package_json["dependencies"]["@vlayer/sdk"].is_null() {
        return warn(&format!("{} not found in {}", "@vlayer/sdk".bold(), "package.json".bold()));
    }
    print_update_intention("@vlayer/sdk");
    package_manager(path).update_vlayer()
}

async fn update_contracts() -> Result<()> {
    let foundry_toml = find_file_up_tree("foundry.toml")?;
    if let Some(mut foundry_toml_path) = foundry_toml {
        foundry_toml_path.pop();
        do_update_contracts(&foundry_toml_path).await
    } else {
        warn(&format!("{} not found. Skipping Soldeer update.", "foundry.toml".bold()))
    }
}

fn update_docker() -> Result<()> {
    let version = newest_vlayer_version()?;

    let docker_compose = find_file_up_tree("docker-compose.devnet.yaml")?;
    if let Some(docker_compose_path) = docker_compose {
        print_update_intention(&format!("vlayer docker images to {}", &version));
        do_update_docker_images(&docker_compose_path, &version)
    } else {
        warn(&format!(
            "{} not found. Skipping docker images update.",
            "docker-compose.devnet.yaml".bold()
        ))
    }
}

// Look at the docker-compose file, and recursively update all included files as well.
fn do_update_docker_images(docker_compose_path: &Path, version: &String) -> Result<()> {
    let yaml_content =
        fs::read_to_string(docker_compose_path).map_err(|e| Error::Upgrade(e.to_string()))?;
    let mut compose: serde_yml::Value =
        serde_yml::from_str(&yaml_content).map_err(|e| Error::Upgrade(e.to_string()))?;

    let replaced = replace_vlayer_docker_image_version(&yaml_content, version)?;
    fs::write(docker_compose_path, replaced)?;

    if let Some(includes) = &mut compose["include"].as_sequence_mut() {
        for include in includes.iter_mut() {
            if let Some(included_filename) = &mut include.as_str() {
                let mut included_path = PathBuf::from(docker_compose_path);
                included_path.set_file_name(included_filename);
                do_update_docker_images(&included_path, version)?;
            }
        }
    }
    Ok(())
}

fn replace_vlayer_docker_image_version(content: &str, version: &String) -> Result<String> {
    regex::Regex::new(r"(image:\s*ghcr\.io/vlayer-xyz/[^:\s]+:)[^\s]+")
        .map(|regex| {
            regex
                .replace_all(content, |captures: &regex::Captures| {
                    format!("{}{}", &captures[1], version)
                })
                .to_string()
        })
        .map_err(|_| {
            Error::Upgrade("Failed to create vlayer docker image replacement regex".to_string())
        })
}

async fn do_update_contracts(foundry_toml_path: &Path) -> Result<()> {
    let version = newest_vlayer_version()?;

    print_update_intention(&format!("vlayer contracts to {}", &version));

    let config = Config::default();
    install_solidity_dependencies(&config.sol_dependencies).await?;

    add_remappings(foundry_toml_path, config.sol_dependencies.values())?;

    Ok(())
}

async fn install_solidity_dependencies(dependencies: &SolDependencies) -> Result<()> {
    for (name, dep) in dependencies.as_ref() {
        if dep.is_local() {
            continue;
        }
        let version = dep
            .version()
            .ok_or(ConfigError::RequiredField("version".into()))?;
        let url = dep.url();
        install(name, &version, url.as_ref()).await?;
    }
    Ok(())
}

fn newest_vlayer_version() -> Result<String> {
    let output = std::process::Command::new("vlayer")
        .arg("--version")
        .output()
        .map_err(into_update_err)?;

    if !output.status.success() {
        return Err(Error::Upgrade(format!(
            "Failed to run newest vlayer: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    String::from_utf8_lossy(&output.stdout)
        .split_ascii_whitespace()
        .nth(1)
        .map(String::from)
        .ok_or(Error::Upgrade("Corrupted vlayer binary".to_string()))
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

    fn install_package(pm_name: &str, install_command: &str) -> Result<()> {
        let exit_status = spawn(pm_name, &[install_command, "@vlayer/sdk"])?;
        ensure_success(exit_status, "@vlayer/sdk")
    }

    pub fn update_vlayer(&self) -> Result<()> {
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

fn find_file_up_tree(name: &str) -> Result<Option<PathBuf>> {
    let mut path = std::env::current_dir().map_err(|e| Error::Upgrade(e.to_string()))?;
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

fn spawn(command: &str, args: &[&str]) -> Result<ExitStatus> {
    std::process::Command::new(command)
        .args(args)
        .spawn()
        .and_then(|mut child| child.wait())
        .map_err(into_update_err)
}

fn ensure_success(exist_status: ExitStatus, package_name: &str) -> Result<()> {
    if exist_status.success() {
        print_successful_update(package_name)
    } else {
        Err(Error::Upgrade(format!("Failed to update {package_name}")))
    }
}

fn warn(message: &str) -> Result<()> {
    println!("{} {}\n", "⚠".yellow().bold(), message);
    Ok(())
}

fn print_update_intention(package_name: &str) {
    println!("📦 Updating {}\n", package_name.bold());
}

fn print_successful_update(package_name: &str) -> Result<()> {
    println!(
        "{} {} updated {}\n",
        "✔".green().bold(),
        package_name.bold(),
        "successfully".green()
    );
    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn into_update_err(e: std::io::Error) -> Error {
    Error::Upgrade(e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_vlayer_docker_images() {
        let original_contents: String = r#"
        include:
          - anvil.yaml
          - vdns.yaml
          - web.yaml
        services:
          vlayer-call-server:
            image: ghcr.io/vlayer-xyz/call_server:latest
            container_name: vlayer-call-server
        "#
        .into();

        let target_version: String = "0.1.0-testing-19991202-1234567".into();
        let new_contents =
            replace_vlayer_docker_image_version(&original_contents, &target_version).unwrap();

        let expected = r#"
        include:
          - anvil.yaml
          - vdns.yaml
          - web.yaml
        services:
          vlayer-call-server:
            image: ghcr.io/vlayer-xyz/call_server:0.1.0-testing-19991202-1234567
            container_name: vlayer-call-server
        "#;

        assert_eq!(new_contents, expected);
    }

    #[test]
    fn test_not_replacing_non_vlayer_docker_images() {
        let original_contents: String = r#"
        services:
          anvil-l1:
            image: ghcr.io/foundry-rs/foundry:latest
            container_name: anvil-l1
          anvil-l2-op:
            image: ghcr.io/foundry-rs/foundry:latest
            container_name: anvil-l2-op
        "#
        .into();

        let target_version: String = "0.1.0-testing-19991202-1234567".into();
        let new_contents =
            replace_vlayer_docker_image_version(&original_contents, &target_version).unwrap();

        assert_eq!(new_contents, original_contents);
    }

    #[test]
    fn test_replacing_multiple_services() {
        let original_contents: String = r#"
        services:
          vdns_server:
            image: ghcr.io/vlayer-xyz/vdns_server:0.0.1-20000101
            container_name: vlayer-vdns-server
            platform: linux/amd64
          vlayer-call-server:
            image: ghcr.io/vlayer-xyz/call_server:0.0.1
            container_name: vlayer-call-server
          new_vlayer_service:
            image: ghcr.io/vlayer-xyz/new_service:latest
            depends_on:
              - vlayer-call-server
        "#
        .into();

        let target_version: String = "0.1.0-testing-19991202-1234567".into();
        let new_contents =
            replace_vlayer_docker_image_version(&original_contents, &target_version).unwrap();

        let expected: String = r#"
        services:
          vdns_server:
            image: ghcr.io/vlayer-xyz/vdns_server:0.1.0-testing-19991202-1234567
            container_name: vlayer-vdns-server
            platform: linux/amd64
          vlayer-call-server:
            image: ghcr.io/vlayer-xyz/call_server:0.1.0-testing-19991202-1234567
            container_name: vlayer-call-server
          new_vlayer_service:
            image: ghcr.io/vlayer-xyz/new_service:0.1.0-testing-19991202-1234567
            depends_on:
              - vlayer-call-server
        "#
        .into();

        assert_eq!(new_contents, expected);
    }

    #[test]
    fn test_update_all_vlayer_devnet_services() {
        let devnet_contents = r#"
        include:
          - anvil.yaml
          - vdns.yaml
          - web.yaml
        services:
          vlayer-call-server:
            depends_on:
              - anvil-l1
              - anvil-l2-op
            image: ghcr.io/vlayer-xyz/call_server:latest
            container_name: vlayer-call-server
            environment:
              RUST_LOG: "info,call_engine=debug"
            command: "--proof fake --host 0.0.0.0 --rpc-url 31337:http://anvil-l1:8545 --rpc-url 31338:http://anvil-l2-op:8545"
            ports:
              - "127.0.0.1:3000:3000"
        "#;

        let anvil_contents = r#"
        services:
          anvil-l1:
            image: ghcr.io/foundry-rs/foundry:latest
            container_name: anvil-l1
            platform: linux/amd64
            command: ["anvil --host 0.0.0.0 --chain-id 31337"]
            ports:
              - "127.0.0.1:8545:8545"
          anvil-l2-op:
            image: ghcr.io/foundry-rs/foundry:latest
            container_name: anvil-l2-op
            platform: linux/amd64
            command: ["anvil --host 0.0.0.0 --chain-id 31338 --optimism -m 'indoor dish desk flag debris potato excuse depart ticket judge         file exit'"]
            ports:
              - "127.0.0.1:8546:8545"
        "#;

        let vdns_contents = r#"
        services:
          vdns_server:
            image: ghcr.io/vlayer-xyz/vdns_server:latest
            container_name: vlayer-vdns-server
            platform: linux/amd64
            environment:
              RUST_LOG: "info,vdns_server=debug"
            command: ["-l", "0.0.0.0:3002"]
            ports:
              - "127.0.0.1:3002:3002"
        "#;

        let web_contents = r#"
        services:
          wsproxy:
            image: jwnmulder/websockify:0.12
            container_name: wsproxy
            platform: linux/amd64
            ports:
              - "127.0.0.1:55688:80"
            command: "80 api.x.com:443"
          notary-server:
            image: ghcr.io/tlsnotary/tlsn/notary-server:v0.1.0-alpha.8
            container_name: notary-server
            ports:
              - "127.0.0.1:7047:7047"
        "#;

        let temp_dir = tempfile::tempdir().unwrap();

        let devnet_path = temp_dir.path().join("docker-compose.devnet.yaml");
        let anvil_path = temp_dir.path().join("anvil.yaml");
        let vdns_path = temp_dir.path().join("vdns.yaml");
        let web_path = temp_dir.path().join("web.yaml");

        std::fs::write(&devnet_path, devnet_contents).unwrap();
        std::fs::write(&anvil_path, anvil_contents).unwrap();
        std::fs::write(&vdns_path, vdns_contents).unwrap();
        std::fs::write(&web_path, web_contents).unwrap();

        let target_version: String = "0.1.0-testing-19991202-11fe464".into();
        do_update_docker_images(&devnet_path, &target_version).unwrap();

        let new_devnet = std::fs::read_to_string(devnet_path).unwrap();
        let new_anvil = std::fs::read_to_string(anvil_path).unwrap();
        let new_vdns = std::fs::read_to_string(vdns_path).unwrap();
        let new_web = std::fs::read_to_string(web_path).unwrap();

        let expected_devnet = r#"
        include:
          - anvil.yaml
          - vdns.yaml
          - web.yaml
        services:
          vlayer-call-server:
            depends_on:
              - anvil-l1
              - anvil-l2-op
            image: ghcr.io/vlayer-xyz/call_server:0.1.0-testing-19991202-11fe464
            container_name: vlayer-call-server
            environment:
              RUST_LOG: "info,call_engine=debug"
            command: "--proof fake --host 0.0.0.0 --rpc-url 31337:http://anvil-l1:8545 --rpc-url 31338:http://anvil-l2-op:8545"
            ports:
              - "127.0.0.1:3000:3000"
        "#;

        let expected_vdns = r#"
        services:
          vdns_server:
            image: ghcr.io/vlayer-xyz/vdns_server:0.1.0-testing-19991202-11fe464
            container_name: vlayer-vdns-server
            platform: linux/amd64
            environment:
              RUST_LOG: "info,vdns_server=debug"
            command: ["-l", "0.0.0.0:3002"]
            ports:
              - "127.0.0.1:3002:3002"
        "#;

        assert_eq!(new_devnet, expected_devnet);
        assert_eq!(new_anvil, anvil_contents);
        assert_eq!(new_vdns, expected_vdns);
        assert_eq!(new_web, web_contents);
    }
}
