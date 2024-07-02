use crate::errors::CLIError;
use crate::misc::parse_toml::get_src_from_string;
use crate::misc::path::find_foundry_root;

pub(crate) fn find_src() -> Result<String, CLIError> {
    let root_path = find_foundry_root()?;
    println!("Found Foundry root -- path: {:?}", root_path);
    let toml_path = root_path.join("foundry.toml");
    let contents = std::fs::read_to_string(toml_path)?;
    get_src_from_string(contents)
}
