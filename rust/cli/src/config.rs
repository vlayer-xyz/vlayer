use std::collections::HashMap;

use clap::ValueEnum;

use crate::target_version;

const VLAYER_FOUNDRY_PKG: SoldeerDependency<'static> = SoldeerDependency {
    name: "vlayer",
    version: "0.1.0",
    url: None,
    remappings: &[("vlayer-0.1.0", "src")],
};

const OPENZEPPELIN_FOUNDRY_PKG: SoldeerDependency<'static> = SoldeerDependency {
    name: "@openzeppelin-contracts",
    version: "5.0.1",
    url: None,
    remappings: &[("openzeppelin-contracts", "")],
};

const FORGE_STD_FOUNDRY_PKG: SoldeerDependency<'static> = SoldeerDependency {
    name: "forge-std",
    version: "1.9.4",
    url: None,
    remappings: &[("forge-std", "src"), ("forge-std-1.9.4/src", "src")],
};

const RISC0_ETHEREUM_FOUNDRY_PKG: SoldeerDependency<'static> = SoldeerDependency{
    name: "risc0-ethereum",
    version: "1.2.0",
    url: Some("https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v1.2.0-soldeer/contracts.zip"),
    remappings: &[("risc0-ethereum-1.2.0", "")],
};

pub const SDK_NPM_NAME: &str = "@vlayer/sdk";
pub const SDK_HOOKS_NPM_NAME: &str = "@vlayer/react";

#[derive(thiserror::Error, Debug)]
#[error("Unresolved config field")]
pub struct UnresolvedError;

pub type Result<T> = std::result::Result<T, UnresolvedError>;

#[derive(Debug)]
pub struct Config {
    pub template: Option<Template>,
    pub contracts: HashMap<String, Dependency>,
    pub npm: HashMap<String, Dependency>,
}

impl Config {
    pub fn template(&self) -> Result<Template> {
        self.template.clone().ok_or(UnresolvedError)
    }

    pub const fn npm(&self) -> &HashMap<String, Dependency> {
        &self.npm
    }
}

impl Default for Config {
    fn default() -> Self {
        let template = Some(Template::Simple);

        let version = target_version();

        let mut contracts = HashMap::new();

        let mut vlayer_dep: DetailedDependency = VLAYER_FOUNDRY_PKG.into();
        vlayer_dep.version = version.clone().into();
        vlayer_dep.remappings = Some(
            VLAYER_FOUNDRY_PKG
                .remappings
                .iter()
                .map(|(source, target)| {
                    add_remapping(VLAYER_FOUNDRY_PKG.name, &version, source, target)
                })
                .collect(),
        );
        contracts.insert(VLAYER_FOUNDRY_PKG.name.into(), Dependency::Detailed(vlayer_dep));

        contracts.insert(OPENZEPPELIN_FOUNDRY_PKG.name.into(), OPENZEPPELIN_FOUNDRY_PKG.into());
        contracts.insert(FORGE_STD_FOUNDRY_PKG.name.into(), FORGE_STD_FOUNDRY_PKG.into());
        contracts.insert(RISC0_ETHEREUM_FOUNDRY_PKG.name.into(), RISC0_ETHEREUM_FOUNDRY_PKG.into());

        let mut npm = HashMap::new();
        npm.insert(SDK_NPM_NAME.into(), Dependency::Simple(version.clone()));
        npm.insert(SDK_HOOKS_NPM_NAME.into(), Dependency::Simple(version));

        Self {
            template,
            contracts,
            npm,
        }
    }
}

#[derive(Debug)]
pub enum Dependency<P: Clone = String> {
    Simple(String),
    Detailed(DetailedDependency<P>),
}

impl<P> Dependency<P>
where
    P: Clone,
{
    pub fn path(&self) -> Result<P> {
        self.as_detailed()
            .and_then(DetailedDependency::path)
            .ok_or(UnresolvedError)
    }

    pub fn version(&self) -> Result<String> {
        match self {
            Self::Simple(version) => Ok(version.clone()),
            Self::Detailed(detailed) => detailed.version().ok_or(UnresolvedError),
        }
    }

    pub fn url(&self) -> Result<String> {
        self.as_detailed()
            .and_then(DetailedDependency::url)
            .ok_or(UnresolvedError)
    }

    pub fn remappings(&self) -> Result<&[(String, P)]> {
        self.as_detailed()
            .and_then(DetailedDependency::remappings)
            .ok_or(UnresolvedError)
    }

    const fn as_detailed(&self) -> Option<&DetailedDependency<P>> {
        match self {
            Self::Simple(..) => None,
            Self::Detailed(x) => Some(x),
        }
    }
}

impl From<SoldeerDependency<'_>> for Dependency<String> {
    fn from(value: SoldeerDependency<'_>) -> Self {
        Self::Detailed(value.into())
    }
}

#[derive(Debug)]
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

pub fn add_remapping(name: &str, version: &str, source: &str, target: &str) -> (String, String) {
    (source.to_string(), format!("dependencies/{name}-{version}/{target}"))
}

impl From<SoldeerDependency<'_>> for DetailedDependency<String> {
    fn from(value: SoldeerDependency<'_>) -> Self {
        let path = None;
        let version = value.version.to_string();
        let url = value.url.map(ToString::to_string);
        let remappings: Vec<(String, String)> = value
            .remappings
            .iter()
            .map(|(x, y)| add_remapping(value.name, value.version, x, y))
            .collect();
        Self {
            path,
            version: Some(version),
            url,
            remappings: Some(remappings),
        }
    }
}

struct SoldeerDependency<'a> {
    name: &'a str,
    version: &'a str,
    url: Option<&'a str>,
    remappings: &'a [(&'a str, &'a str)],
}

#[derive(Clone, Debug, ValueEnum, Default)]
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
