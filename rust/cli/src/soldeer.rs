use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use soldeer_commands::{
    commands::{install::Install, Command},
    run as run_cmd, ConfigLocation,
};

use crate::{config::Dependency, errors::Result};

pub async fn install(name: &String, version: &String, url: Option<&String>) -> Result<()> {
    let cmd = Install::builder()
        .dependency(format!("{name}~{version}"))
        .maybe_remote_url(url)
        .config_location(ConfigLocation::Foundry)
        .build();
    run_cmd(Command::Install(cmd)).await?;
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
    let mut all_remappings = filter_existing_remappings(&remappings_path, &keys)?;

    let mut remappings: Vec<String> = remappings.iter().map(|(x, y)| format!("{x}={y}")).collect();
    all_remappings.append(&mut remappings);
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
