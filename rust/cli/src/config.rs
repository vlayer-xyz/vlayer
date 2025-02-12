use std::collections::HashMap;

use crate::target_version;

const FOUNDRY_PKG_NAME: &str = "vlayer";
const SDK_NPM_NAME: &str = "@vlayer/sdk";
const SDK_HOOKS_NPM_NAME: &str = "@vlayer/react";

pub struct Config {
    pub contracts: HashMap<String, Dependency>,
    pub npm: HashMap<String, Dependency>,
}

impl Default for Config {
    fn default() -> Self {
        let version = target_version();
        let mut contracts = HashMap::new();
        contracts.insert(FOUNDRY_PKG_NAME.into(), Dependency::Simple(version.clone()));
        let mut npm = HashMap::new();
        npm.insert(SDK_NPM_NAME.into(), Dependency::Simple(version.clone()));
        npm.insert(SDK_HOOKS_NPM_NAME.into(), Dependency::Simple(version));
        Self { contracts, npm }
    }
}

pub enum Dependency<P: Clone = String> {
    Simple(String),
    Detailed(DetailedDependency<P>),
}

pub struct DetailedDependency<P: Clone = String> {
    pub path: Option<P>,
}
