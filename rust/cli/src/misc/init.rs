use crate::errors::CLIError;
use crate::misc::parse_toml::get_src_from_string;
use std::path::{Path, PathBuf};

const VLAYER_DIR_NAME: &str = "vlayer";

pub(crate) fn find_src_path(root_path: PathBuf) -> Result<PathBuf, CLIError> {
    let toml_path = root_path.join("foundry.toml");
    let contents = std::fs::read_to_string(toml_path)?;
    let src_dirname = get_src_from_string(contents)?;
    let src_path = root_path.join(src_dirname);
    if src_path.exists() {
        Ok(src_path)
    } else {
        Err(CLIError::SrcDirNotFound(src_path))
    }
}

pub(crate) fn create_vlayer_dir(src_path: &Path) -> Result<bool, CLIError> {
    let vlayer_dir = src_path.join(VLAYER_DIR_NAME);
    if vlayer_dir.exists() {
        return Ok(false);
    } else {
        std::fs::create_dir_all(&vlayer_dir)?;
    }
    Ok(true)
}
