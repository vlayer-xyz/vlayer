use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::Output,
};

use lazy_static::lazy_static;

use crate::{commands::version::version, errors::CLIError};

lazy_static! {
    pub(crate) static ref DEPENDENCIES: Vec<SoldeerDep> = vec![
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

pub(crate) fn add_remappings(foundry_root: &Path, deps: &[SoldeerDep]) -> Result<(), CLIError> {
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

#[derive(Clone)]
pub(crate) struct SoldeerDep {
    pub name: String,
    pub version: String,
    pub url: Option<String>,
    pub remapping: Option<Remapping>,
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

#[derive(Clone)]
pub(crate) struct Remapping {
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

fn build_new_remappings(deps: &[SoldeerDep]) -> (Vec<String>, Vec<String>) {
    deps.iter()
        .filter_map(SoldeerDep::remapping)
        .flatten()
        .unzip()
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
