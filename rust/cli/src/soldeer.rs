use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use soldeer_commands::{
    ConfigLocation,
    commands::{Command, install::Install},
    run as run_cmd,
};

use crate::{
    config::{self, Dependency, SolDependencies},
    errors::Result,
};

pub async fn install(name: &String, version: &String, url: Option<&String>) -> Result<()> {
    let cmd = Install::builder()
        .dependency(format!("{name}~{version}"))
        .maybe_remote_url(url)
        .config_location(ConfigLocation::Foundry)
        .build();
    run_cmd(Command::Install(cmd)).await?;
    Ok(())
}

pub async fn install_solidity_dependencies(dependencies: &SolDependencies) -> Result<()> {
    for (name, dep) in dependencies.as_ref() {
        if dep.is_local() {
            continue;
        }
        let version = dep
            .version()
            .ok_or(config::Error::RequiredField("version".into()))?;
        let url = dep.url();
        install(name, &version, url.as_ref()).await?;
    }
    Ok(())
}

pub fn add_remappings<'a>(
    foundry_root: &Path,
    iter: impl Iterator<Item = &'a Dependency>,
) -> Result<()> {
    let remappings: Vec<(String, String)> = iter
        .flat_map(Dependency::remappings)
        .flatten()
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect();
    do_add_remappings(foundry_root, &remappings)
}

fn do_add_remappings(foundry_root: &Path, remappings: &[(String, String)]) -> Result<()> {
    let remappings_path = foundry_root.join("remappings.txt");

    let keys: Vec<String> = remappings.iter().map(|(x, _)| x.clone()).collect();
    let all_remappings = filter_existing_remappings(&remappings_path, &keys)?;

    let mut all_remappings: Vec<String> = all_remappings
        .into_iter()
        .chain(remappings.iter().map(|(x, y)| format!("{x}={y}")))
        .collect();
    all_remappings.sort();

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&remappings_path)?;

    writeln!(file, "{}", all_remappings.join("\n"))?;

    Ok(())
}

fn filter_existing_remappings(remappings_path: &PathBuf, keys: &[String]) -> Result<Vec<String>> {
    let remappings = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(remappings_path)?;
    let curr_remappings = BufReader::new(remappings).lines();
    let matches_no_key = |line: &String| keys.iter().all(|key| !line.starts_with(key));
    let filtered_remappings = curr_remappings
        .map_while(std::result::Result::ok)
        .filter(matches_no_key)
        .collect();
    Ok(filtered_remappings)
}
