use std::{collections::HashMap, fmt};

use clap::ValueEnum;
use serde::Deserialize;

use crate::version;

pub const DEFAULT_CONFIG: &str = "
template = 'simple'
[contracts.vlayer]
version = '0.1.0'
remappings = [['vlayer-0.1.0/', 'dependencies/vlayer-VERSION/src/']]
[contracts.'@openzeppelin-contracts']
version = '5.0.1'
remappings = [['openzeppelin-contracts/', 'dependencies/@openzeppelin-contracts-5.0.1/']]
[contracts.forge-std]
version = '1.9.4'
remappings = [
  ['forge-std/', 'dependencies/forge-std-1.9.4/src/'],
  ['forge-std-1.9.4/src/', 'dependencies/forge-std-1.9.4/src/']
]
[contracts.risc0-ethereum]
version = '1.2.0'
url = 'https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v1.2.0-soldeer/contracts.zip'
remappings = [['risc0-ethereum-1.2.0/', 'dependencies/risc0-ethereum-1.2.0/']]
[npm]
'@vlayer/sdk' = 'VERSION'
'@vlayer/react' = 'VERSION'
";

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Missing required config field: {0}")]
    RequiredField(String),
    #[error("Failed loading from TOML: {0}")]
    Toml(#[from] toml::de::Error),
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
        toml::from_str(str.as_ref()).map_err(Error::Toml)
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
}

impl Default for Config {
    fn default() -> Self {
        let version = version();
        toml::from_str(&DEFAULT_CONFIG.replace("VERSION", &version))
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
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct DetailedDependency<P: Clone = String> {
    pub path: Option<P>,
    pub version: Option<String>,
    pub url: Option<String>,
    pub remappings: Option<Vec<(String, P)>>,
}

impl<P> DetailedDependency<P>
where
    P: Clone,
{
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
}
