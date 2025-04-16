use std::{collections::BTreeMap, fmt, fs};

use anyhow::Context;
use clap::ValueEnum;
use derive_more::{AsRef, Deref, DerefMut};
use serde::Deserialize;
use thiserror::Error;

use crate::version;

pub const DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing required config field: {0}")]
    RequiredField(String),
    #[error("Failed loading from TOML: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Invalid path as remapping target: '/'")]
    InvalidRemappingTarget,
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(AsRef, Deref, DerefMut, Deserialize, Debug)]
pub struct SolDependencies<P: Clone = String>(pub BTreeMap<String, Dependency<P>>);

#[derive(AsRef, Deref, DerefMut, Deserialize, Debug)]
pub struct JsDependencies<P: Clone = String>(pub BTreeMap<String, Dependency<P>>);

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub template: Option<Template>,
    pub sol_dependencies: SolDependencies,
    pub js_dependencies: JsDependencies,
}

impl Config {
    pub fn from_str(str: impl AsRef<str>) -> Result<Self> {
        let mut config: Self = toml::from_str(str.as_ref())?;
        config.canonicalize()?;
        Ok(config)
    }

    pub fn template(&self) -> Result<Template> {
        self.template
            .clone()
            .ok_or(Error::RequiredField("template".into()))
    }

    pub fn canonicalize(&mut self) -> Result<()> {
        self.sol_dependencies
            .values_mut()
            .filter_map(Dependency::as_detailed_mut)
            .try_for_each(DetailedDependency::canonicalize)?;

        self.js_dependencies
            .values_mut()
            .filter_map(Dependency::as_detailed_mut)
            .try_for_each(DetailedDependency::canonicalize)?;

        Ok(())
    }
}

impl Default for Config {
    #![allow(clippy::expect_used)]
    fn default() -> Self {
        Self::from_str(DEFAULT_CONFIG.replace("{{VERSION}}", &version()))
            .expect("default config cannot be malformed")
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Dependency<P: Clone = String> {
    Simple(String),
    Detailed(DetailedDependency<P>),
}

impl<P: Clone> Dependency<P> {
    pub fn path(&self) -> Option<P> {
        self.as_detailed().and_then(DetailedDependency::path)
    }

    pub fn version(&self) -> Option<String> {
        match self {
            Self::Simple(version) => Some(version.clone()),
            Self::Detailed(detailed) => detailed.version(),
        }
    }

    pub fn url(&self) -> Option<String> {
        self.as_detailed().and_then(DetailedDependency::url)
    }

    pub fn remappings(&self) -> Result<&[(String, P)]> {
        self.as_detailed()
            .and_then(DetailedDependency::remappings)
            .ok_or(Error::RequiredField("remappings".into()))
    }

    pub fn is_local(&self) -> bool {
        self.path().is_some()
    }

    pub const fn as_detailed(&self) -> Option<&DetailedDependency<P>> {
        match self {
            Self::Simple(..) => None,
            Self::Detailed(x) => Some(x),
        }
    }

    pub fn as_detailed_mut(&mut self) -> Option<&mut DetailedDependency<P>> {
        match self {
            Self::Simple(..) => None,
            Self::Detailed(x) => Some(x),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct DetailedDependency<P: Clone = String> {
    pub path: Option<P>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub remappings: Option<Vec<(String, P)>>,
}

impl<P: Clone> DetailedDependency<P> {
    pub fn path(&self) -> Option<P> {
        self.path.clone()
    }

    pub fn version(&self) -> Option<String> {
        self.version.clone()
    }

    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }

    pub fn remappings(&self) -> Option<&[(String, P)]> {
        self.remappings.as_deref()
    }
}

impl DetailedDependency<String> {
    fn canonicalize(&mut self) -> Result<()> {
        self.path = if let Some(ref mut path) = self.path {
            let path =
                fs::canonicalize(path).with_context(|| "Failed to canonicalize path '{path}'")?;
            Some(path.to_string_lossy().into_owned())
        } else {
            None
        };
        Ok(())
    }
}

#[derive(Clone, Debug, ValueEnum, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Template {
    #[default]
    Simple,
    SimpleEmailProof,
    SimpleTeleport,
    SimpleTimeTravel,
    SimpleWebProof,
    KrakenWebProof,
}

impl fmt::Display for Template {
    #[allow(clippy::expect_used)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_value = self
            .to_possible_value()
            .expect("no Template variant should be skipped");
        let name = as_value.get_name();
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_parse_simple_config() {
        let config = Config::from_str(
            "
template = 'simple-web-proof'

[sol-dependencies]
vlayer = { version='0.0.1', remappings = [['abc/', 'dependencies/abc/']] }

[js-dependencies]
'@vlayer/sdk' = '0.0.1'
            ",
        )
        .unwrap();

        assert_eq!(config.template().unwrap(), Template::SimpleWebProof);

        {
            assert!(config.sol_dependencies.contains_key("vlayer"));

            let dep = config
                .sol_dependencies
                .get("vlayer")
                .unwrap()
                .as_detailed()
                .unwrap();

            assert!(dep.path.is_none());
            assert!(dep.url.is_none());
            assert_eq!(dep.version.clone().unwrap(), "0.0.1");

            let remappings = dep.remappings.clone().unwrap();
            assert_eq!(remappings, &[("abc/".into(), "dependencies/abc/".into())]);
        }
    }

    #[test]
    fn test_missing_template_field() {
        let config = Config::from_str(
            "
[sol-dependencies]

[js-dependencies]
            ",
        )
        .unwrap();

        assert!(
            matches!(config.template().err().unwrap(), Error::RequiredField(field) if field == "template")
        );
    }

    #[test]
    fn test_missing_remappings_for_solidity_dep() {
        let config = Config::from_str(
            "
[sol-dependencies]
vlayer = '0.0.1'

[js-dependencies]
            ",
        )
        .unwrap();

        assert!(
            matches!(config.sol_dependencies.get("vlayer").unwrap().remappings().err().unwrap(), Error::RequiredField(field) if field == "remappings")
        );
    }

    #[test]
    fn test_default_config() {
        let version = version();
        let config = Config::default();

        assert_eq!(config.template().unwrap(), Template::Simple);
        assert_eq!(config.sol_dependencies.len(), 4);

        {
            let dep = config.sol_dependencies.get("vlayer").unwrap();
            assert_eq!(dep.version().unwrap(), version);
            assert_eq!(
                dep.remappings().unwrap(),
                &[("vlayer-0.1.0/".into(), format!("dependencies/vlayer-{version}/src/"))]
            );
        }

        {
            let dep = config
                .sol_dependencies
                .get("@openzeppelin-contracts")
                .unwrap();
            assert_eq!(dep.version().unwrap(), "5.0.1");
            assert_eq!(
                dep.remappings().unwrap(),
                &[(
                    "openzeppelin-contracts/".into(),
                    "dependencies/@openzeppelin-contracts-5.0.1/".into()
                )]
            );
        }

        {
            let dep = config.sol_dependencies.get("forge-std").unwrap();
            assert_eq!(dep.version().unwrap(), "1.9.4");
            assert_eq!(
                dep.remappings().unwrap(),
                &[
                    ("forge-std/".into(), "dependencies/forge-std-1.9.4/src/".into()),
                    ("forge-std-1.9.4/src/".into(), "dependencies/forge-std-1.9.4/src/".into())
                ]
            );
        }

        {
            let dep = config.sol_dependencies.get("risc0-ethereum").unwrap();
            assert_eq!(dep.version().unwrap(), "2.0.0");
            assert_eq!(
                dep.remappings().unwrap(),
                &[("risc0-ethereum-2.0.0/".into(), "dependencies/risc0-ethereum-2.0.0/".into())]
            );
        }

        assert_eq!(config.js_dependencies.len(), 2);

        assert_eq!(
            config
                .js_dependencies
                .get("@vlayer/sdk")
                .unwrap()
                .version()
                .unwrap(),
            version
        );
        assert_eq!(
            config
                .js_dependencies
                .get("@vlayer/react")
                .unwrap()
                .version()
                .unwrap(),
            version
        );
    }

    #[test]
    fn test_paths_are_canonicalized() {
        let raw_config = "
template = 'simple'

[sol-dependencies]
vlayer = { path='../vlayer', remappings = [['abc/', 'dependencies/abc/']] }

[js-dependencies]
'@vlayer/sdk' = { path = '../vlayer' }
            ";

        let temp_dir = tempdir().unwrap();
        let cwd = temp_dir.path();
        let vlayer = cwd.join("vlayer");
        fs::create_dir(&vlayer).unwrap();

        let other = cwd.join("other");
        fs::create_dir(&other).unwrap();

        std::env::set_current_dir(other).unwrap();

        let config = Config::from_str(raw_config).unwrap();

        // On macOS, /var/tmp may be a symlink to /private/var/tmp, so we fuzzy match.
        let dep = config.sol_dependencies.get("vlayer").unwrap();
        assert!(dep.path().unwrap().contains(vlayer.to_str().unwrap()));

        assert!(
            config
                .js_dependencies
                .get("@vlayer/sdk")
                .unwrap()
                .path()
                .unwrap()
                .contains(vlayer.to_str().unwrap())
        );
    }
}
