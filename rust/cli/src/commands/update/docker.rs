use std::{
    fs, io,
    path::{Path, PathBuf},
};

use colored::Colorize;
use thiserror::Error;

use super::logger::UpdateLogger;
use crate::{cli_wrappers::vlayer, utils::path::find_file_up_tree};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to create vlayer docker image replacement regex: {0}")]
    Regex(#[from] regex::Error),
    #[error("Failed to find dockerfile: {0}")]
    FindDockerfile(#[from] anyhow::Error),
    #[error("Docker compose read failed: {0}")]
    DockerComposeRead(io::Error),
    #[error("Docker compose write failed: {0}")]
    DockerComposeWrite(io::Error),
    #[error("Docker compose parse failed: {0}")]
    DockerComposeParse(serde_yml::Error),
    #[error(transparent)]
    Vlayer(#[from] vlayer::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn update_docker() -> Result<()> {
    let version = vlayer::Cli::version()?;
    let logger = UpdateLogger::new(format!("Vlayer docker to {}", &version));
    let docker_compose = find_file_up_tree("docker-compose.devnet.yaml")?;
    let Some(docker_compose_path) = docker_compose else {
        logger.warn(format!(
            "{} not found. Skipping docker images update.",
            "docker-compose.devnet.yaml".bold()
        ));
        return Ok(());
    };

    do_update_docker_images(&docker_compose_path, &version)?;
    logger.success();

    Ok(())
}

// Look at the docker-compose file, and recursively update all included files as well.
fn do_update_docker_images(docker_compose_path: &Path, version: &String) -> Result<()> {
    let yaml_content = fs::read_to_string(docker_compose_path).map_err(Error::DockerComposeRead)?;
    let mut compose: serde_yml::Value =
        serde_yml::from_str(&yaml_content).map_err(Error::DockerComposeParse)?;

    let replaced = replace_vlayer_docker_image_version(&yaml_content, version)?;
    fs::write(docker_compose_path, replaced).map_err(Error::DockerComposeWrite)?;

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
    let re = regex::Regex::new(r"(image:\s*ghcr\.io/vlayer-xyz/[^:\s]+:)[^\s]+")?;
    Ok(re
        .replace_all(content, |captures: &regex::Captures| format!("{}{}", &captures[1], version))
        .to_string())
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
              - "127.0.0.1:3003:80" 
            command: "80 api.x.com:443"
          notary-server:
            image: ghcr.io/tlsnotary/tlsn/notary-server:v0.1.0-alpha.7
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
