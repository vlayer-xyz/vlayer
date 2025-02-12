use std::{
    collections::HashMap,
    convert::TryFrom,
    fs::{self, OpenOptions},
    io::{Cursor, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Context;
use clap::Parser;
use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use reqwest::get;
use serde_json::{Map, Value};
use tar::Archive;
use tracing::{error, info};

use crate::{
    commands::common::soldeer::{add_remappings, DEPENDENCIES},
    config::{Config, Dependency, Template, UnresolvedError, SDK_HOOKS_NPM_NAME, SDK_NPM_NAME},
    errors::CLIError,
    target_version,
    utils::{
        parse_toml::{add_deps_to_foundry_toml, get_src_from_str},
        path::{copy_dir_to, find_foundry_root},
    },
};

#[derive(Clone, Debug, Parser)]
pub(crate) struct Args {
    /// Template to use for the project
    #[arg(long, value_enum)]
    pub(crate) template: Option<Template>,
    /// Force init in existing project location
    #[arg(long)]
    pub(crate) existing: bool,
    /// Name of the project
    #[arg()]
    pub(crate) project_name: Option<String>,
    /// Directory where the templates will be unpacked into (useful for debugging)
    #[arg(long, env = "VLAYER_WORK_DIR")]
    pub(crate) work_dir: Option<PathBuf>,
}

const VLAYER_DIR_NAME: &str = "vlayer";

enum WorkDir {
    Temp(tempfile::TempDir),
    Explicit(PathBuf),
}

impl WorkDir {
    fn path(&self) -> &Path {
        match self {
            Self::Temp(dir) => dir.path(),
            Self::Explicit(path) => path,
        }
    }
}

impl TryFrom<Option<PathBuf>> for WorkDir {
    type Error = anyhow::Error;

    fn try_from(value: Option<PathBuf>) -> Result<Self, Self::Error> {
        match value {
            Some(path) => {
                fs::create_dir_all(&path)
                    .with_context(|| format!("Failed to create work dir '{}'", path.display()))?;
                Ok(Self::Explicit(path))
            }
            None => {
                let tempdir =
                    tempfile::tempdir().context("Failed to create work dir in temp files")?;
                Ok(Self::Temp(tempdir))
            }
        }
    }
}

lazy_static! {
    static ref EXAMPLES_URL: String = format!(
        "https://vlayer-releases.s3.eu-north-1.amazonaws.com/{}/examples.tar.gz",
        target_version()
    );
}

fn install_dependencies(foundry_root: &Path) -> Result<(), CLIError> {
    for dep in DEPENDENCIES.iter() {
        dep.install(foundry_root)?;
    }

    Ok(())
}

fn add_default_remappings(foundry_root: &Path) -> Result<(), CLIError> {
    add_remappings(foundry_root, DEPENDENCIES.as_slice())
}

fn change_sdk_dependency_to_npm(
    foundry_root: &Path,
    deps: &HashMap<String, Dependency>,
) -> Result<(), CLIError> {
    let package_json = foundry_root.join("vlayer").join("package.json");
    let mut file = OpenOptions::new().read(true).open(package_json.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut json: Value = serde_json::from_str(&contents)?;

    let mut dependencies_map = json
        .get("dependencies")
        .and_then(serde_json::Value::as_object)
        .map_or(Map::new(), Clone::clone);

    dependencies_map.insert(
        SDK_NPM_NAME.into(),
        Value::String(
            deps.get(SDK_NPM_NAME)
                .ok_or(UnresolvedError)
                .and_then(Dependency::version)?,
        ),
    );

    if dependencies_map.contains_key(SDK_HOOKS_NPM_NAME) {
        dependencies_map.insert(
            SDK_HOOKS_NPM_NAME.into(),
            Value::String(
                deps.get(SDK_HOOKS_NPM_NAME)
                    .ok_or(UnresolvedError)
                    .and_then(Dependency::version)?,
            ),
        );
    }
    json["dependencies"] = Value::Object(dependencies_map);

    let new_contents = serde_json::to_string_pretty(&json)?;
    fs::write(package_json, new_contents)?;

    Ok(())
}

pub(crate) async fn run_init(args: Args) -> Result<(), CLIError> {
    let mut cwd = std::env::current_dir()?;

    let mut config = Config::default();
    config.template = args.template.or(config.template);
    println!("{config:#?}");

    if !args.existing {
        let mut command = std::process::Command::new("forge");
        command.arg("init");
        if let Some(project_name) = &args.project_name {
            cwd.push(project_name);
            command.arg(project_name);
        }
        let output = command.output().with_context(|| match args.project_name {
            Some(project_name) => format!("Invoking 'forge init {project_name}' failed"),
            None => "Invoking 'forge init' failed".to_string(),
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CLIError::ForgeInitError(stderr.to_string()));
        }
    }

    let work_dir = args.work_dir.try_into()?;
    init_existing(cwd, config, work_dir).await
}

async fn init_existing(cwd: PathBuf, config: Config, work_dir: WorkDir) -> Result<(), CLIError> {
    info!("Running vlayer init from directory {:?}", cwd.display());

    let root_path = find_foundry_root(&cwd)?;
    let src_path = find_src_path(&root_path)?;

    info!("Found foundry project root in \"{}\"", &root_path.display());

    if vlayer_dir_exists_in(&src_path) || vlayer_dir_exists_in(&root_path) {
        error!(
            "vlayer directory already exists in \"{}\" or \"{}\". Skipping creation.",
            &root_path.display(),
            &src_path.display()
        )
    } else {
        let scripts_dst = create_vlayer_dir(&root_path)?;
        let examples_dst = create_vlayer_dir(&src_path)?;
        let tests_dst = create_vlayer_dir(&root_path.join("test"))?;
        let testdata_dst = root_path.join("testdata");
        let template = config.template()?;
        fetch_examples(
            &examples_dst,
            &scripts_dst,
            &tests_dst,
            &testdata_dst,
            template.to_string(),
            work_dir,
        )
        .await?;
        info!("Successfully downloaded vlayer template \"{}\"", template);
    }

    add_deps_to_foundry_toml(&root_path)?;

    std::env::set_current_dir(&root_path)?;

    add_fs_permissions_to_foundry_toml(&root_path)?;

    info!("Initialising soldeer");
    init_soldeer(&root_path)?;

    info!("Installing dependencies");
    install_dependencies(&root_path)?;
    info!("Successfully installed all dependencies");
    add_default_remappings(&root_path)?;

    change_sdk_dependency_to_npm(&root_path, config.npm())?;

    std::env::set_current_dir(&cwd)?;

    Ok(())
}

fn add_fs_permissions_to_foundry_toml(root_path: &Path) -> Result<(), std::io::Error> {
    let foundry_toml_path = root_path.join("foundry.toml");
    append_file(
        &foundry_toml_path,
        "fs_permissions = [{ access = \"read\", path = \"./testdata\"}]",
    )
}

fn init_soldeer(root_path: &Path) -> Result<(), CLIError> {
    add_deps_to_gitignore(root_path)?;
    add_deps_section_to_foundry_toml(root_path)?;
    add_soldeer_config_to_foundry_toml(root_path)?;

    Ok(())
}

fn add_deps_section_to_foundry_toml(root_path: &Path) -> Result<(), std::io::Error> {
    let foundry_toml_path = root_path.join("foundry.toml");
    append_file(&foundry_toml_path, "\n[dependencies]")
}

fn add_soldeer_config_to_foundry_toml(root_path: &Path) -> Result<(), std::io::Error> {
    let foundry_toml_path = root_path.join("foundry.toml");
    let soldeer_section = "
[soldeer]
# whether soldeer manages remappings
remappings_generate = false
# whether soldeer re-generates all remappings when installing, updating or uninstalling deps
remappings_regenerate = false
";

    append_file(&foundry_toml_path, soldeer_section)
}

fn add_deps_to_gitignore(root_path: &Path) -> Result<(), std::io::Error> {
    let gitignore_path = root_path.join(".gitignore");
    append_file(&gitignore_path, "**/dependencies/")
}

fn append_file(file: &Path, suffix: &str) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new().append(true).open(file)?;

    writeln!(file, "{suffix}")?;

    Ok(())
}

const fn map_reqwest_error(e: reqwest::Error) -> CLIError {
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
    tests_dst: &Path,
    testdata_dst: &Path,
    template: String,
    work_dir: WorkDir,
) -> Result<(), CLIError> {
    let response = get(EXAMPLES_URL.as_str())
        .await
        .map_err(map_reqwest_error)?
        .bytes()
        .await
        .map_err(map_reqwest_error)?;

    let mut archive = Archive::new(GzDecoder::new(Cursor::new(response)));

    let work_dir_path = work_dir.path();
    archive.unpack(work_dir_path)?;

    let downloaded_scripts = work_dir_path.join(&template).join("vlayer");
    let downloaded_examples = work_dir_path.join(&template).join("src/vlayer");
    let downloaded_tests = work_dir_path.join(&template).join("test/vlayer");
    let downloaded_testdata = work_dir_path.join(&template).join("testdata");

    copy_dir_to(&downloaded_scripts, scripts_dst)?;
    copy_dir_to(&downloaded_examples, examples_dst)?;

    // not all examples come with tests and testdata
    if downloaded_tests.exists() {
        copy_dir_to(&downloaded_tests, tests_dst)?;
    }
    if downloaded_testdata.exists() {
        copy_dir_to(&downloaded_testdata, testdata_dst)?;
    }

    Ok(())
}

pub(crate) fn vlayer_dir_exists_in(src_path: &Path) -> bool {
    src_path.join(VLAYER_DIR_NAME).exists()
}

pub(crate) fn create_vlayer_dir(src_path: &Path) -> anyhow::Result<PathBuf> {
    let vlayer_dir = src_path.join(VLAYER_DIR_NAME);
    std::fs::create_dir_all(&vlayer_dir)
        .with_context(|| format!("Failed to create path {}", vlayer_dir.display()))?;
    info!("Created vlayer directory in \"{}\"", src_path.display());
    Ok(vlayer_dir)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::{build_version, test_utils::create_temp_git_repo};

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
        std::fs::write(root_path.join(".gitignore"), "").unwrap(); // empty gitignore
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

        add_default_remappings(&root_path).unwrap();

        let remappings_txt = root_path.join("remappings.txt");
        let contents = fs::read_to_string(remappings_txt).unwrap();

        let expected_remappings = format!(
            "some initial remappings\n\
            openzeppelin-contracts/=dependencies/@openzeppelin-contracts-5.0.1/\n\
            forge-std/=dependencies/forge-std-1.9.4/src/\n\
            forge-std-1.9.4/src/=dependencies/forge-std-1.9.4/src/\n\
            risc0-ethereum-1.2.0/=dependencies/risc0-ethereum-1.2.0/\n\
            vlayer-0.1.0/=dependencies/vlayer-{}/src/\n",
            build_version()
        );

        assert_eq!(contents, expected_remappings);
    }

    #[test]
    fn test_upsert_remapping() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();
        let remappings_txt = root_path.join("remappings.txt");
        std::fs::write(&remappings_txt, "vlayer-0.1.0/=dependencies/vlayer-0.0.0/src/\n").unwrap();

        add_default_remappings(&root_path).unwrap();

        let remappings_txt = root_path.join("remappings.txt");
        let contents = fs::read_to_string(remappings_txt).unwrap();

        let expected_remappings = format!(
            "openzeppelin-contracts/=dependencies/@openzeppelin-contracts-5.0.1/\n\
            forge-std/=dependencies/forge-std-1.9.4/src/\n\
            forge-std-1.9.4/src/=dependencies/forge-std-1.9.4/src/\n\
            risc0-ethereum-1.2.0/=dependencies/risc0-ethereum-1.2.0/\n\
            vlayer-0.1.0/=dependencies/vlayer-{}/src/\n",
            build_version()
        );

        assert_eq!(contents, expected_remappings);
    }

    #[test]
    fn test_dont_add_react_sdk_dependency_to_package_json_if_not_already_present() {
        let config = Config::default();
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        let vlayer_dir = root_path.join("vlayer");
        std::fs::create_dir(&vlayer_dir).unwrap();

        let package_json = vlayer_dir.join("package.json");
        let contents = r#"{"dependencies": {}}"#;
        std::fs::write(&package_json, contents).unwrap();

        change_sdk_dependency_to_npm(&root_path, config.npm()).unwrap();

        let new_contents = fs::read_to_string(package_json).unwrap();
        let expected_sdk_dependency = format!("\"@vlayer/react\": \"{}\"", build_version());
        assert!(!new_contents.contains(&expected_sdk_dependency));
    }

    #[test]
    fn test_change_workspace_react_sdk_dependency_to_npm() {
        let config = Config::default();
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        let vlayer_dir = root_path.join("vlayer");
        std::fs::create_dir(&vlayer_dir).unwrap();

        let package_json = vlayer_dir.join("package.json");
        let contents = r#"{"dependencies": {"@vlayer/react": "workspace:*"}}"#;
        std::fs::write(&package_json, contents).unwrap();

        change_sdk_dependency_to_npm(&root_path, config.npm()).unwrap();

        let new_contents = fs::read_to_string(package_json).unwrap();
        let expected_sdk_dependency = format!("\"@vlayer/react\": \"{}\"", build_version());
        assert!(new_contents.contains(&expected_sdk_dependency));
    }

    #[test]
    fn test_add_sdk_dependency_to_package_json() {
        let config = Config::default();
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        let vlayer_dir = root_path.join("vlayer");
        std::fs::create_dir(&vlayer_dir).unwrap();

        let package_json = vlayer_dir.join("package.json");
        let contents = r#"{"dependencies": {}}"#;
        std::fs::write(&package_json, contents).unwrap();

        change_sdk_dependency_to_npm(&root_path, config.npm()).unwrap();

        let new_contents = fs::read_to_string(package_json).unwrap();
        let expected_sdk_dependency = format!("\"@vlayer/sdk\": \"{}\"", build_version());
        assert!(new_contents.contains(&expected_sdk_dependency));
    }

    #[test]
    fn test_change_workspace_sdk_dependency_to_npm() {
        let config = Config::default();
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        let vlayer_dir = root_path.join("vlayer");
        std::fs::create_dir(&vlayer_dir).unwrap();

        let package_json = vlayer_dir.join("package.json");
        let contents = r#"{"dependencies": {"@vlayer/sdk": "workspace:*"}}"#;
        std::fs::write(&package_json, contents).unwrap();

        change_sdk_dependency_to_npm(&root_path, config.npm()).unwrap();

        let new_contents = fs::read_to_string(package_json).unwrap();
        let expected_sdk_dependency = format!("\"@vlayer/sdk\": \"{}\"", build_version());

        assert!(!new_contents.contains("file:../../../packages/sdk"));
        assert!(new_contents.contains(&expected_sdk_dependency));
    }

    #[test]
    fn test_adding_fs_permissions_to_foundry_toml() {
        let (_temp_dir, _, root_path) = prepare_foundry_dir("src");

        add_fs_permissions_to_foundry_toml(&root_path).unwrap();

        let foundry_toml = fs::read_to_string(root_path.join("foundry.toml")).unwrap();
        assert!(
            foundry_toml.contains("fs_permissions = [{ access = \"read\", path = \"./testdata\"}]")
        );
    }

    mod init_soldeer {
        use super::*;

        #[test]
        fn adds_dependencies_dir_to_gitignore() {
            let (_temp_dir, _, root_path) = prepare_foundry_dir("src");

            init_soldeer(&root_path).unwrap();

            let gitignore = fs::read_to_string(root_path.join(".gitignore")).unwrap();
            assert!(gitignore.contains("**/dependencies/"));
        }

        #[test]
        fn adds_dependencies_section_to_foundry_toml() {
            let (_temp_dir, _, root_path) = prepare_foundry_dir("src");

            init_soldeer(&root_path).unwrap();

            let foundry_toml = fs::read_to_string(root_path.join("foundry.toml")).unwrap();
            assert!(foundry_toml.contains("[dependencies]"));
        }

        #[test]
        fn adds_soldeer_section_to_foundry_toml() {
            let (_temp_dir, _, root_path) = prepare_foundry_dir("src");

            init_soldeer(&root_path).unwrap();

            let foundry_toml = fs::read_to_string(root_path.join("foundry.toml")).unwrap();

            assert!(foundry_toml.contains("[soldeer]"));
            assert!(foundry_toml.contains("remappings_generate = false"));
            assert!(foundry_toml.contains("remappings_regenerate = false"));
        }
    }
}
