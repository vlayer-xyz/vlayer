use std::collections::HashMap;

use clap::ValueEnum;

use crate::target_version;

pub const FOUNDRY_PKG_NAME: &str = "vlayer";
pub const SDK_NPM_NAME: &str = "@vlayer/sdk";
pub const SDK_HOOKS_NPM_NAME: &str = "@vlayer/react";

#[derive(thiserror::Error, Debug)]
#[error("Unresolved config field")]
pub struct UnresolvedError;

pub type Result<T> = std::result::Result<T, UnresolvedError>;

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
        contracts.insert(FOUNDRY_PKG_NAME.into(), Dependency::Simple(version.clone()));

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

pub enum Dependency<P: Clone = String> {
    Simple(String),
    Detailed(DetailedDependency<P>),
}

impl<P> Dependency<P>
where
    P: Clone,
{
    pub fn version(&self) -> Result<String> {
        match self {
            Self::Simple(version) => Ok(version.clone()),
            Self::Detailed(DetailedDependency { version, .. }) => {
                version.clone().ok_or(UnresolvedError)
            }
        }
    }
}

pub struct DetailedDependency<P: Clone = String> {
    pub path: Option<P>,
    pub version: Option<String>,
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
