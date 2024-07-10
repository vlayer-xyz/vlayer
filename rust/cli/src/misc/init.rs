use crate::errors::CLIError;
use crate::misc::parse_toml::get_src_from_string;
use flate2::read::GzDecoder;
use reqwest::get;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tar::Archive;

const VLAYER_DIR_NAME: &str = "vlayer";
const CONTRACTS_URL: &str =
    "https://vlayer-releases.s3.eu-north-1.amazonaws.com/latest/contracts.tar.gz";

fn map_reqwest_error(e: reqwest::Error) -> CLIError {
    CLIError::DownloadVlayerFilesError(e)
}

pub(crate) fn find_src_path(root_path: &Path) -> Result<PathBuf, CLIError> {
    let toml_path = root_path.join("foundry.toml");
    let contents = fs::read_to_string(toml_path)?;
    let src_dirname = get_src_from_string(contents)?;
    let src_path = root_path.join(src_dirname);
    if src_path.exists() {
        Ok(src_path)
    } else {
        Err(CLIError::SrcDirNotFound(src_path))
    }
}

pub(crate) async fn fetch_vlayer_files(dst: &Path) -> Result<(), CLIError> {
    let response = get(CONTRACTS_URL)
        .await
        .map_err(map_reqwest_error)?
        .bytes()
        .await
        .map_err(map_reqwest_error)?;

    Archive::new(GzDecoder::new(Cursor::new(response))).unpack(dst)?;

    Ok(())
}

pub(crate) fn create_vlayer_dir(src_path: &Path) -> Result<bool, CLIError> {
    let vlayer_dir = src_path.join(VLAYER_DIR_NAME);
    if vlayer_dir.exists() {
        return Ok(false);
    }
    std::fs::create_dir_all(&vlayer_dir)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::test_utils::create_temp_git_repo;

    fn prepare_empty_foundry_dir(src_name: &str) -> TempDir {
        // creates a temporary directory with a foundry.toml file
        // without(!) an src directory
        let temp_dir = create_temp_git_repo();
        let toml_path = temp_dir.path().join("foundry.toml");
        let s = format!(
            r#"
          [profile.default]
          src = "{src_name}"
          "#
        );
        std::fs::write(toml_path, s).unwrap();
        temp_dir
    }

    fn prepare_foundry_dir(src_name: &str) -> (TempDir, PathBuf, PathBuf) {
        // creates a temporary directory with a foundry.toml file
        // and an src directory
        let temp_dir = prepare_empty_foundry_dir(src_name);
        let root_path = temp_dir.path().to_path_buf();
        let src_dir_path = root_path.join(src_name);
        std::fs::create_dir_all(&src_dir_path).unwrap();
        (temp_dir, src_dir_path, root_path)
    }

    fn test_find_src_path(src: &str) {
        let (_temp_dir, src_path, root_path) = prepare_foundry_dir(src);
        let path = find_src_path(&root_path).unwrap();
        assert_eq!(path, src_path);
    }

    #[test]
    fn test_find_src_path_simple() {
        test_find_src_path("src");
    }

    #[test]
    fn test_find_src_path_long() {
        test_find_src_path("my/path/to/source");
    }

    #[test]
    fn test_find_src_path_nonexistent() {
        let temp_dir = prepare_empty_foundry_dir("src");
        let root_path = temp_dir.path().to_path_buf();

        let result = find_src_path(&root_path);

        let e = result.unwrap_err();
        assert!(matches!(
            e,
            CLIError::SrcDirNotFound(
                path
            ) if path == root_path.join("src")
        ));
    }

    #[test]
    fn test_create_vlayer_dir() {
        let (_temp_dir, src_path, _root_path) = prepare_foundry_dir("src");

        let vlayer_dir = src_path.join(VLAYER_DIR_NAME);

        let result = create_vlayer_dir(&src_path);

        assert!(result.unwrap());
        assert!(vlayer_dir.exists());
    }

    #[test]
    fn test_create_vlayer_dir_alr_exists() {
        let (_temp_dir, src_path, _root_path) = prepare_foundry_dir("src");

        let vlayer_dir = src_path.join(VLAYER_DIR_NAME);
        std::fs::create_dir_all(&vlayer_dir).unwrap();

        let result = create_vlayer_dir(&src_path);

        assert!(!result.unwrap());
        assert!(vlayer_dir.exists());
    }
}
