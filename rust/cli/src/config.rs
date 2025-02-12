use std::collections::HashMap;

use clap::ValueEnum;

use crate::target_version;

const FOUNDRY_PKG_NAME: &str = "vlayer";
const SDK_NPM_NAME: &str = "@vlayer/sdk";
const SDK_HOOKS_NPM_NAME: &str = "@vlayer/react";

pub struct Config {
    pub template: Option<Template>,
    pub contracts: HashMap<String, Dependency>,
    pub npm: HashMap<String, Dependency>,
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

pub struct DetailedDependency<P: Clone = String> {
    pub path: Option<P>,
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
