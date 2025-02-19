use std::{collections::HashMap, fmt, fs};

use anyhow::Context;
use clap::ValueEnum;
use serde::Deserialize;

use crate::version;

pub const DEFAULT_CONFIG: &str = include_str!("../config.toml");

#[derive(thiserror::Error, Debug)]
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub template: Option<Template>,
    pub contracts: HashMap<String, Dependency>,
    pub npm: HashMap<String, Dependency>,
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

    pub const fn npm(&self) -> &HashMap<String, Dependency> {
        &self.npm
    }

    pub const fn contracts(&self) -> &HashMap<String, Dependency> {
        &self.contracts
    }

    pub fn canonicalize(&mut self) -> Result<()> {
        self.contracts
            .values_mut()
            .filter_map(Dependency::as_detailed_mut)
            .try_for_each(DetailedDependency::canonicalize)?;

        self.npm
            .values_mut()
            .filter_map(Dependency::as_detailed_mut)
            .try_for_each(DetailedDependency::canonicalize)?;

        Ok(())
    }
}

impl Default for Config {
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

impl<P> Dependency<P>
where
    P: Clone,
{
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
}

impl fmt::Display for Template {
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

[contracts]
vlayer = { version='0.0.1', remappings = [['abc/', 'dependencies/abc/']] }

[npm]
'@vlayer/sdk' = '0.0.1'
            ",
        )
        .unwrap();

        assert!(config.template().is_ok());
        assert_eq!(config.template().unwrap(), Template::SimpleWebProof);

        {
            assert!(config.contracts.contains_key("vlayer"));

            let dep = config.contracts.get("vlayer").unwrap();
            assert!(matches!(dep, Dependency::Detailed(..)));

            let Dependency::Detailed(DetailedDependency {
                path,
                version,
                url,
                remappings,
            }) = dep
            else {
                unreachable!();
            };

            assert!(path.is_none());
            assert!(url.is_none());
            assert_eq!(version.clone().unwrap(), "0.0.1");

            let remappings = remappings.clone().unwrap();
            assert_eq!(remappings.len(), 1);
            assert_eq!(remappings[0], ("abc/".into(), "dependencies/abc/".into()));
        }
    }

    #[test]
    fn test_missing_template_field() {
        let config = Config::from_str(
            "
[contracts]

[npm]
            ",
        )
        .unwrap();

        assert!(config.template().is_err());
        assert!(
            matches!(config.template().err().unwrap(), Error::RequiredField(field) if field == "template")
        );
    }

    #[test]
    fn test_missing_remappings_for_contract_dep() {
        let config = Config::from_str(
            "
[contracts]
vlayer = '0.0.1'

[npm]
            ",
        )
        .unwrap();

        assert!(config.contracts.contains_key("vlayer"));
        assert!(
            matches!(config.contracts.get("vlayer").unwrap().remappings().err().unwrap(), Error::RequiredField(field) if field == "remappings")
        );
    }

    #[test]
    fn test_default_config() {
        let version = version();
        let config = Config::default();

        assert_eq!(config.template().unwrap(), Template::Simple);

        let contracts = config.contracts();
        assert_eq!(contracts.len(), 4);

        assert!(contracts.contains_key("vlayer"));
        {
            let dep = contracts.get("vlayer").unwrap();
            assert_eq!(dep.version().unwrap(), "0.1.0");
            assert_eq!(
                dep.remappings().unwrap(),
                &[("vlayer-0.1.0/".into(), format!("dependencies/vlayer-{version}/src/"))]
            );
        }

        assert!(contracts.contains_key("@openzeppelin-contracts"));
        {
            let dep = contracts.get("@openzeppelin-contracts").unwrap();
            assert_eq!(dep.version().unwrap(), "5.0.1");
            assert_eq!(
                dep.remappings().unwrap(),
                &[(
                    "openzeppelin-contracts/".into(),
                    "dependencies/@openzeppelin-contracts-5.0.1/".into()
                )]
            );
        }

        assert!(contracts.contains_key("forge-std"));
        {
            let dep = contracts.get("forge-std").unwrap();
            assert_eq!(dep.version().unwrap(), "1.9.4");
            assert_eq!(
                dep.remappings().unwrap(),
                &[
                    ("forge-std/".into(), "dependencies/forge-std-1.9.4/src/".into()),
                    ("forge-std-1.9.4/src/".into(), "dependencies/forge-std-1.9.4/src/".into())
                ]
            );
        }

        assert!(contracts.contains_key("risc0-ethereum"));
        {
            let dep = contracts.get("risc0-ethereum").unwrap();
            assert_eq!(dep.version().unwrap(), "1.2.0");
            assert_eq!(
                dep.remappings().unwrap(),
                &[("risc0-ethereum-1.2.0/".into(), "dependencies/risc0-ethereum-1.2.0/".into())]
            );
        }

        let npm = config.npm();
        assert_eq!(npm.len(), 2);

        assert!(npm.contains_key("@vlayer/sdk"));
        assert_eq!(npm.get("@vlayer/sdk").unwrap().version().unwrap(), version);
        assert!(npm.contains_key("@vlayer/react"));
        assert_eq!(npm.get("@vlayer/react").unwrap().version().unwrap(), version);
    }

    #[test]
    fn test_paths_are_canonicalized() {
        let raw_config = "
template = 'simple'

[contracts]
vlayer = { path='../vlayer', remappings = [['abc/', 'dependencies/abc/']] }

[npm]
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

        let contracts = config.contracts();
        assert!(contracts.contains_key("vlayer"));
        {
            let dep = contracts.get("vlayer").unwrap();
            assert_eq!(dep.path().unwrap(), vlayer.to_string_lossy());
        }

        let npm = config.npm();
        assert!(npm.contains_key("@vlayer/sdk"));
        assert_eq!(npm.get("@vlayer/sdk").unwrap().path().unwrap(), vlayer.to_string_lossy());
    }
}
