use std::collections::HashMap;

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
#[error("Unresolved config field: {0}")]
pub struct UnresolvedError(pub String);

pub type Result<T> = std::result::Result<T, UnresolvedError>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub template: Option<Template>,
    pub contracts: HashMap<String, Dependency>,
    pub npm: HashMap<String, Dependency>,
}

impl Config {
    pub fn template(&self) -> Result<Template> {
        self.template
            .clone()
            .ok_or(UnresolvedError("template".into()))
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
            .ok_or(UnresolvedError("remappings".into()))
    }

    pub const fn as_detailed(&self) -> Option<&DetailedDependency<P>> {
        match self {
            Self::Simple(..) => None,
            Self::Detailed(x) => Some(x),
        }
    }
}

#[derive(Debug, Deserialize)]
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

#[derive(Clone, Debug, ValueEnum, Default, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Template {
    #[default]
    Simple,
    SimpleEmailProof,
    SimpleTeleport,
    SimpleTimeTravel,
    SimpleWebProof,
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_value = self
            .to_possible_value()
            .expect("no Template variant should be skipped");
        let name = as_value.get_name();
        write!(f, "{name}")
    }
}
