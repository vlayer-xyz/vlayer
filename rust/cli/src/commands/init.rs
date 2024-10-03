use std::{
    fs,
    fs::OpenOptions,
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
    process::Output,
};

use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use reqwest::get;
use serde_json::{Map, Value};
use tar::Archive;
use tracing::{error, info};

use crate::{
    commands::{args::TemplateOption, version::version},
    errors::CLIError,
    utils::{
        parse_toml::{add_deps_to_foundry_toml, get_src_from_str},
        path::{copy_dir_to, find_foundry_root},
    },
};

const VLAYER_DIR_NAME: &str = "vlayer";
const EXAMPLES_URL: &str =
    "https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/examples.tar.gz";

lazy_static! {
    static ref DEPENDENCIES: Vec<SoldeerDep> = vec![
        SoldeerDep {
            name: String::from("@openzeppelin-contracts"),
            version: String::from("5.0.1"),
            url: None,
            remapping: Some("openzeppelin-contracts".into()),
        },
        SoldeerDep {
            name: String::from("forge-std"),
            version: String::from("1.9.2"),
            url: None,
            remapping: Some(("forge-std", "src").into()),
        },
        SoldeerDep {
            name: String::from("risc0-ethereum"),
            version: String::from("1.0.0"),
            url: Some(String::from("https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v1.0.0-soldeer-no-remappings/contracts.zip")),
            remapping: None,
        },
        SoldeerDep {
            name: String::from("vlayer"),
            version: version(),
            url: None,
            remapping: Some(("vlayer-0.1.0", "src").into() ),
        }
    ];
}

struct SoldeerDep {
    name: String,
    version: String,
    url: Option<String>,
    remapping: Option<Remapping>,
}

struct Remapping {
    key: String,
    internal_path: Option<String>,
}

impl Remapping {
    fn new(key: &str, internal_path: Option<&str>) -> Self {
        let internal_path = internal_path.map(ToString::to_string);
        Self {
            key: key.to_string(),
            internal_path,
        }
    }
}

impl From<(&str, &str)> for Remapping {
    fn from(value: (&str, &str)) -> Self {
        let (key, internal_path) = value;
        Remapping::new(key, Some(internal_path))
    }
}
impl From<&str> for Remapping {
    fn from(value: &str) -> Self {
        Remapping::new(value, None)
    }
}

impl SoldeerDep {
    pub fn install(&self) -> Result<(), CLIError> {
        let output = match &self.url {
            Some(url) => Self::install_url_dep(&self.name, &self.version, url)?,
            None => Self::install_dep(&self.name, &self.version)?,
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CLIError::ForgeInitError(stderr.to_string()));
        }

        Ok(())
    }

    fn install_dep(name: &String, version: &String) -> Result<Output, CLIError> {
        let output = std::process::Command::new("forge")
            .arg("soldeer")
            .arg("install")
            .arg(format!("{}~{}", name, version))
            .output()?;

        Ok(output)
    }

    fn install_url_dep(name: &String, version: &String, url: &String) -> Result<Output, CLIError> {
        let output = std::process::Command::new("forge")
            .arg("soldeer")
            .arg("install")
            .arg(format!("{}~{}", name, version))
            .arg(url)
            .output()?;

        Ok(output)
    }

    fn remapping(&self) -> Option<String> {
        let remapping = self.remapping.as_ref()?;
        let internal_path = if let Some(internal_path) = &remapping.internal_path {
            format!("{}/", internal_path)
        } else {
            String::default()
        };

        Some(format!(
            "{}/=dependencies/{}-{}/{}",
            remapping.key, self.name, self.version, internal_path
        ))
    }
}

fn install_dependencies() -> Result<(), CLIError> {
    for dep in DEPENDENCIES.iter() {
        dep.install()?;
    }

    Ok(())
}

fn add_remappings(foundry_root: &Path) -> Result<(), CLIError> {
    let remappings_txt = foundry_root.join("remappings.txt");
    let mut file = OpenOptions::new().append(true).open(remappings_txt)?;
    for dep in DEPENDENCIES.iter() {
        if let Some(remapping) = dep.remapping() {
            writeln!(file, "{remapping}")?;
        }
    }

    Ok(())
}

fn change_sdk_dependency_to_npm(foundry_root: &Path) -> Result<(), CLIError> {
    let package_json = foundry_root.join("vlayer").join("package.json");
    let mut file = OpenOptions::new().read(true).open(package_json.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut json: Value = serde_json::from_str(&contents)?;
    let version = version();

    if let Some(dependencies) = json.get_mut("dependencies") {
        if let Some(dependencies_map) = dependencies.as_object_mut() {
            dependencies_map.insert("@vlayer/sdk".to_string(), Value::String(version));
        }
    } else {
        let mut dependencies_map = Map::new();
        dependencies_map.insert("@vlayer/sdk".to_string(), Value::String(version));
        json["dependencies"] = Value::Object(dependencies_map);
    }

    let new_contents = serde_json::to_string_pretty(&json)?;
    fs::write(package_json, new_contents)?;

    Ok(())
}

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

    info!("Installing dependencies");
    install_dependencies()?;
    info!("Successfully installed all dependencies");
    add_remappings(&root_path)?;

    change_sdk_dependency_to_npm(&root_path)?;

    std::env::set_current_dir(&cwd)?;

    Ok(())
}

fn map_reqwest_error(e: reqwest::Error) -> CLIError {
    CLIError::DownloadExamplesError(e)
}

fn find_src_path(root_path: &Path) -> Result<PathBuf, CLIError> {
    let toml_path = root_path.join("foundry.toml");
    let contents = fs::read_to_string(toml_path)?;
    let src_dirname = get_src_from_str(contents)?;
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

    #[test]
    fn test_add_remappings() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();
        let remappings_txt = root_path.join("remappings.txt");
        std::fs::write(&remappings_txt, "some initial remappings\n").unwrap();

        add_remappings(&root_path).unwrap();

        let remappings_txt = root_path.join("remappings.txt");
        let contents = fs::read_to_string(remappings_txt).unwrap();

        let expected_remappings = format!(
            "some initial remappings\n\
            openzeppelin-contracts/=dependencies/@openzeppelin-contracts-5.0.1/\n\
            forge-std/=dependencies/forge-std-1.9.2/src/\n\
            vlayer-0.1.0/=dependencies/vlayer-{}/src/\n",
            version()
        );

        assert_eq!(contents, expected_remappings);
    }
}
