use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Cursor, Read, Write},
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
    commands::{
        args::{InitArgs, TemplateOption},
        version::version,
    },
    errors::CLIError,
    utils::{
        parse_toml::{add_deps_to_foundry_toml, get_src_from_str},
        path::{copy_dir_to, find_foundry_root},
    },
};

const VLAYER_DIR_NAME: &str = "vlayer";

lazy_static! {
    static ref EXAMPLES_URL: String = format!("https://vlayer-releases.s3.eu-north-1.amazonaws.com/{}/examples.tar.gz", version());

    static ref DEPENDENCIES: Vec<SoldeerDep> = vec![
        SoldeerDep {
            name: "@openzeppelin-contracts".into(),
            version: "5.0.1".into(),
            url: None,
            remapping: Some("openzeppelin-contracts".into()),
        },
        SoldeerDep {
            name: "forge-std".into(),
            version: "1.9.2".into(),
            url: None,
            remapping: Some((["forge-std", "forge-std-1.9.2/src"].as_slice(), "src").into()),
        },
        SoldeerDep {
            name: "risc0-ethereum".into(),
            version: "1.1.4".into(),
            url: Some("https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v1.1.4-soldeer/contracts.zip".into()),
            remapping: Some("risc0-ethereum-1.1.4".into()),
        },
        SoldeerDep {
            name: "vlayer".into(),
            version: version(),
            url: None,
            remapping: Some(("vlayer-0.1.0", "src").into() ),
        }
    ];
}

pub(super) struct SoldeerDep {
    pub name: String,
    pub version: String,
    pub url: Option<String>,
    pub remapping: Option<Remapping>,
}

#[derive(Debug, Clone)]
enum Key {
    Single(String),
    Multi(Vec<String>),
}

impl From<&str> for Key {
    fn from(value: &str) -> Self {
        Key::Single(value.into())
    }
}

impl From<&[&str]> for Key {
    fn from(value: &[&str]) -> Self {
        Key::Multi(value.iter().map(ToString::to_string).collect())
    }
}

pub(super) struct Remapping {
    key: Key,
    internal_path: Option<String>,
}

impl Remapping {
    fn new(key: Key, internal_path: Option<&str>) -> Self {
        let internal_path = internal_path.map(ToString::to_string);
        Self { key, internal_path }
    }
}

impl From<(&str, &str)> for Remapping {
    fn from(value: (&str, &str)) -> Self {
        let (key, internal_path) = value;
        Remapping::new(key.into(), Some(internal_path))
    }
}

impl From<(&[&str], &str)> for Remapping {
    fn from(value: (&[&str], &str)) -> Self {
        let (key, internal_path) = value;
        Remapping::new(key.into(), Some(internal_path))
    }
}

impl From<&str> for Remapping {
    fn from(key: &str) -> Self {
        Remapping::new(key.into(), None)
    }
}

impl SoldeerDep {
    pub fn install(&self, foundry_root: &Path) -> Result<(), CLIError> {
        let output = match &self.url {
            Some(url) => Self::install_url_dep(foundry_root, &self.name, &self.version, url)?,
            None => Self::install_dep(foundry_root, &self.name, &self.version)?,
        };

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CLIError::ForgeInitError(stderr.to_string()));
        }

        Ok(())
    }

    fn install_dep(
        foundry_root: &Path,
        name: &String,
        version: &String,
    ) -> Result<Output, CLIError> {
        let output = std::process::Command::new("forge")
            .arg("soldeer")
            .arg("install")
            .arg(format!("{name}~{version}"))
            .current_dir(foundry_root)
            .output()?;

        Ok(output)
    }

    fn install_url_dep(
        foundry_root: &Path,
        name: &String,
        version: &String,
        url: &String,
    ) -> Result<Output, CLIError> {
        let output = std::process::Command::new("forge")
            .arg("soldeer")
            .arg("install")
            .arg(format!("{name}~{version}"))
            .arg(url)
            .current_dir(foundry_root)
            .output()?;

        Ok(output)
    }

    fn remapping(&self) -> Option<Vec<(String, String)>> {
        let remapping = self.remapping.as_ref()?;
        let internal_path = if let Some(internal_path) = &remapping.internal_path {
            format!("{internal_path}/")
        } else {
            String::default()
        };

        let key = remapping.key.clone();
        let dependency = format!("dependencies/{}-{}/{}", self.name, self.version, internal_path);
        let remappings = match key {
            Key::Single(key) => vec![(key.clone(), format!("{key}/={dependency}"))],
            Key::Multi(keys) => keys
                .iter()
                .map(|key| (key.clone(), format!("{key}/={dependency}")))
                .collect(),
        };

        Some(remappings)
    }
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

pub(super) fn add_remappings(foundry_root: &Path, deps: &[SoldeerDep]) -> Result<(), CLIError> {
    let remappings_path = foundry_root.join("remappings.txt");

    let (keys, mut new_remappings) = build_new_remappings(deps);
    let mut remappings = filter_existing_remappings(&remappings_path, &keys)?;

    remappings.append(&mut new_remappings);

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&remappings_path)?;

    writeln!(file, "{}", remappings.join("\n"))?;

    Ok(())
}

fn filter_existing_remappings(
    remappings_path: &PathBuf,
    keys: &[String],
) -> Result<Vec<String>, CLIError> {
    let remappings = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(remappings_path)?;
    let curr_remappings = BufReader::new(remappings).lines();
    let matches_no_key = |line: &String| keys.iter().all(|key| !line.starts_with(key));
    let filtered_remappings = curr_remappings
        .map_while(Result::ok)
        .filter(matches_no_key)
        .collect();
    Ok(filtered_remappings)
}

fn build_new_remappings(deps: &[SoldeerDep]) -> (Vec<String>, Vec<String>) {
    deps.iter()
        .filter_map(SoldeerDep::remapping)
        .flatten()
        .unzip()
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

pub(crate) async fn run_init(args: InitArgs) -> Result<(), CLIError> {
    let mut cwd = std::env::current_dir()?;

    if !args.existing {
        let mut command = std::process::Command::new("forge");
        command.arg("init");
        if let Some(project_name) = args.project_name {
            cwd.push(&project_name);
            command.arg(project_name);
        }
        let output = command.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(CLIError::ForgeInitError(stderr.to_string()));
        }
    }

    init_existing(cwd, args.template.unwrap_or_default()).await
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
        let tests_dst = create_vlayer_dir(&root_path.join("test"))?;
        let testdata_dst = root_path.join("testdata");
        fetch_examples(
            &examples_dst,
            &scripts_dst,
            &tests_dst,
            &testdata_dst,
            template.to_string(),
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

    change_sdk_dependency_to_npm(&root_path)?;

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
) -> Result<(), CLIError> {
    let response = get(EXAMPLES_URL.as_str())
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
    let downloaded_tests = temp_dir.path().join(&template).join("test/vlayer");
    let downloaded_testdata = temp_dir.path().join(&template).join("testdata");

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
            forge-std/=dependencies/forge-std-1.9.2/src/\n\
            forge-std-1.9.2/src/=dependencies/forge-std-1.9.2/src/\n\
            risc0-ethereum-1.1.4/=dependencies/risc0-ethereum-1.1.4/\n\
            vlayer-0.1.0/=dependencies/vlayer-{}/src/\n",
            version()
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
            forge-std/=dependencies/forge-std-1.9.2/src/\n\
            forge-std-1.9.2/src/=dependencies/forge-std-1.9.2/src/\n\
            risc0-ethereum-1.1.4/=dependencies/risc0-ethereum-1.1.4/\n\
            vlayer-0.1.0/=dependencies/vlayer-{}/src/\n",
            version()
        );

        assert_eq!(contents, expected_remappings);
    }

    #[test]
    fn test_change_sdk_dependency_to_npm() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        let vlayer_dir = root_path.join("vlayer");
        std::fs::create_dir(&vlayer_dir).unwrap();

        let package_json = vlayer_dir.join("package.json");
        let contents = r#"{"dependencies": {}}"#;
        std::fs::write(&package_json, contents).unwrap();

        change_sdk_dependency_to_npm(&root_path).unwrap();

        let new_contents = fs::read_to_string(package_json).unwrap();
        let expected_sdk_dependency = format!("\"@vlayer/sdk\": \"{}\"", version());
        assert!(new_contents.contains(&expected_sdk_dependency));
    }

    #[test]
    fn test_change_sdk_dependency_to_npm_with_existing_dependency() {
        let temp_dir = tempfile::tempdir().unwrap();
        let root_path = temp_dir.path().to_path_buf();

        let vlayer_dir = root_path.join("vlayer");
        std::fs::create_dir(&vlayer_dir).unwrap();

        let package_json = vlayer_dir.join("package.json");
        let contents = r#"{"dependencies": {"@vlayer/sdk": "file:../../../packages/sdk"}}"#;
        std::fs::write(&package_json, contents).unwrap();

        change_sdk_dependency_to_npm(&root_path).unwrap();

        let new_contents = fs::read_to_string(package_json).unwrap();
        let expected_sdk_dependency = format!("\"@vlayer/sdk\": \"{}\"", version());

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
