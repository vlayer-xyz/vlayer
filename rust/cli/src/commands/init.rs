use crate::commands::args::TemplateOption;
use crate::errors::CLIError;
use crate::utils::parse_toml::{add_deps_to_foundry_toml, get_src_from_string};
use crate::utils::path::{copy_dir_to, find_foundry_root};
use flate2::read::GzDecoder;
use reqwest::get;
use std::fs;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Write;
use std::path::{Path, PathBuf};
use tar::Archive;
use tracing::{error, info};

const VLAYER_DIR_NAME: &str = "vlayer";
const EXAMPLES_URL: &str =
    "https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/examples.tar.gz";
const CONTRACTS_URL: &str =
    "https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/contracts.zip";

const VLAYER_PACKAGE: &str = "vlayer~0.1.0";

pub(crate) async fn init(
    mut cwd: PathBuf,
    template: TemplateOption,
    existing: bool,
    project_name: Option<String>,
) -> Result<(), CLIError> {
    if !existing {
        let mut command = std::process::Command::new("forge");
        command.arg("init");
        if let Some(project_name) = project_name {
            cwd.push(&project_name);
            command.arg(project_name);
        }
        let output = command.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CLIError::ForgeInitError(stderr.to_string()));
        }
    }

    init_existing(cwd, template).await
}

pub(crate) async fn init_existing(cwd: PathBuf, template: TemplateOption) -> Result<(), CLIError> {
    info!("Running vlayer init from directory {:?}", cwd.display());

    let root_path = find_foundry_root(&cwd)?;
    let src_path = find_src_path(&root_path)?;

    info!("Found foundry project root in \"{}\"", &src_path.display());

    if vlayer_dir_exists_in(&src_path) || vlayer_dir_exists_in(&root_path) {
        error!(
            "vlayer directory already exists in \"{}\" or \"{}\". Skipping creation.",
            &root_path.display(),
            &src_path.display()
        )
    } else {
        let scripts_dst = create_vlayer_dir(&root_path)?;
        let examples_dst = create_vlayer_dir(&src_path)?;
        fetch_examples(&examples_dst, &scripts_dst, template.to_string()).await?;
        info!("Successfully downloaded vlayer template \"{}\"", template);
    }

    add_deps_to_foundry_toml(&root_path)?;

    std::env::set_current_dir(&root_path)?;

    install_contracts()?;
    info!("Successfully installed vlayer contracts");
    install_dependencies()?;
    info!("Successfully installed all dependencies");
    add_risc0_eth_remappings(&root_path)?;

    std::env::set_current_dir(&cwd)?;

    Ok(())
}

fn map_reqwest_error(e: reqwest::Error) -> CLIError {
    CLIError::DownloadExamplesError(e)
}

fn find_src_path(root_path: &Path) -> Result<PathBuf, CLIError> {
    let toml_path = root_path.join("foundry.toml");
    let contents = fs::read_to_string(toml_path)?;
    let src_dirname = get_src_from_string(contents)?;
    let src_path = root_path.join(src_dirname);
    if src_path.exists() {
        Ok(src_path)
    } else {
        Err(CLIError::SrcDirNotFound(src_path))
    }
}

async fn fetch_examples(
    examples_dst: &Path,
    scripts_dst: &Path,
    template: String,
) -> Result<(), CLIError> {
    let response = get(EXAMPLES_URL)
        .await
        .map_err(map_reqwest_error)?
        .bytes()
        .await
        .map_err(map_reqwest_error)?;

    let mut archive = Archive::new(GzDecoder::new(Cursor::new(response)));

    let temp_dir = tempfile::tempdir()?;
    archive.unpack(temp_dir.path())?;

    let downloaded_scripts = temp_dir.path().join(&template).join("vlayer");
    let downloaded_examples = temp_dir.path().join(&template).join("src/vlayer");

    copy_dir_to(&downloaded_scripts, scripts_dst)?;
    copy_dir_to(&downloaded_examples, examples_dst)?;

    Ok(())
}

fn install_contracts() -> Result<(), CLIError> {
    let output = std::process::Command::new("forge")
        .arg("soldeer")
        .arg("install")
        .arg(VLAYER_PACKAGE)
        .arg(CONTRACTS_URL)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CLIError::ForgeInitError(stderr.to_string()));
    }
    Ok(())
}

fn add_risc0_eth_remappings(foundry_root: &Path) -> std::io::Result<()> {
    let remappings_txt = foundry_root.join("remappings.txt");
    let suffix = "forge-std/=dependencies/forge-std-1.8.2/src\nopenzeppelin-contracts=dependencies/@openzeppelin-contracts-5.0.1/";

    let mut file = OpenOptions::new().append(true).open(remappings_txt)?;

    writeln!(file, "{}", suffix)?;

    Ok(())
}

fn install_dependencies() -> Result<(), CLIError> {
    let dependencies = vec!["@openzeppelin-contracts~5.0.1", "forge-std~1.8.2"];

    for dep in dependencies {
        let output = std::process::Command::new("forge")
            .arg("soldeer")
            .arg("install")
            .arg(dep)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CLIError::ForgeInitError(stderr.to_string()));
        }
    }

    let output = std::process::Command::new("forge")
    .arg("soldeer")
    .arg("install")
    .arg("risc0-ethereum~1.0.0")
    .arg("https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v1.0.0-soldeer/contracts.zip")
    .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CLIError::ForgeInitError(stderr.to_string()));
    }

    Ok(())
}

pub(crate) fn vlayer_dir_exists_in(src_path: &Path) -> bool {
    src_path.join(VLAYER_DIR_NAME).exists()
}

pub(crate) fn create_vlayer_dir(src_path: &Path) -> Result<PathBuf, CLIError> {
    let vlayer_dir = src_path.join(VLAYER_DIR_NAME);
    std::fs::create_dir_all(&vlayer_dir)?;
    info!("Created vlayer directory in \"{}\"", src_path.display());
    Ok(vlayer_dir)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::test_utils::create_temp_git_repo;

    fn prepare_empty_foundry_dir(src_name: &str) -> TempDir {
        // creates a temporary directory with a foundry.toml file
        // without(!) an src directory
        let temp_dir = create_temp_git_repo();
        let toml_path = temp_dir.path().join("foundry.toml");
        let s = format!(
            r#"
          [profile.default]
          src = "{src_name}"
          "#
        );
        std::fs::write(toml_path, s).unwrap();
        temp_dir
    }

    fn prepare_foundry_dir(src_name: &str) -> (TempDir, PathBuf, PathBuf) {
        // creates a temporary directory with a foundry.toml file
        // and an src directory
        let temp_dir = prepare_empty_foundry_dir(src_name);
        let root_path = temp_dir.path().to_path_buf();
        let src_dir_path = root_path.join(src_name);
        std::fs::create_dir_all(&src_dir_path).unwrap();
        (temp_dir, src_dir_path, root_path)
    }

    fn test_find_src_path(src: &str) {
        let (_temp_dir, src_path, root_path) = prepare_foundry_dir(src);
        let path = find_src_path(&root_path).unwrap();
        assert_eq!(path, src_path);
    }

    #[test]
    fn test_find_src_path_simple() {
        test_find_src_path("src");
    }

    #[test]
    fn test_find_src_path_long() {
        test_find_src_path("my/path/to/source");
    }

    #[test]
    fn test_find_src_path_nonexistent() {
        let temp_dir = prepare_empty_foundry_dir("src");
        let root_path = temp_dir.path().to_path_buf();

        let result = find_src_path(&root_path);

        let e = result.unwrap_err();
        assert!(matches!(
            e,
            CLIError::SrcDirNotFound(
                path
            ) if path == root_path.join("src")
        ));
    }

    #[test]
    fn test_create_vlayer_dir() {
        let (_temp_dir, src_path, _root_path) = prepare_foundry_dir("src");

        let vlayer_dir = src_path.join(VLAYER_DIR_NAME);

        let result = create_vlayer_dir(&src_path);

        assert!(&vlayer_dir.exists());
        assert_eq!(result.unwrap(), vlayer_dir);
    }
}
