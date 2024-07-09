use crate::errors::CLIError;
use crate::misc::parse_toml::get_src_from_string;
use flate2::read::GzDecoder;
use fs::DirEntry;
use reqwest::get;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::{fs, io};

#[cfg(test)]
use tempfile::TempDir;

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

// fn has_conflict(entry_result: &io::Result<DirEntry>, src_vlayer_path: &Path) -> io::Result<bool> {
//     let entry = match entry_result {
//         Ok(entry) => entry,
//         Err(e) => return Err(io::Error::new(e.kind(), e.to_string())),
//     };
//     let file_name = entry.file_name();
//     let src_file_path = src_vlayer_path.join(&file_name);
//     if src_file_path.exists() {
//         return Ok(true);
//     }
//     Ok(false)
// }

// fn find_conflicts(src: &Path, dst: &Path) -> io::Result<Vec<DirEntry>> {
//     let src_iter = fs::read_dir(src)?;
//     let dst_iter = fs::read_dir(dst)?;

//     let conflicts = src_iter.filter(|entry| has_conflict(entry, dst));

//     conflicts.collect()
// }

fn has_conflict(entry: &DirEntry, src_vlayer_path: &Path) -> bool {
    let file_name = entry.file_name();
    let src_file_path = src_vlayer_path.join(file_name);
    src_file_path.exists()
}

fn check_entry_for_conflict(entry_result: io::Result<DirEntry>, dst: &Path) -> Option<DirEntry> {
    match entry_result {
        Ok(entry) => {
            if has_conflict(&entry, dst) {
                Some(entry)
            } else {
                None
            }
        }
        Err(_) => None, // Skip entries that resulted in an error
                        // if there was an error, it should be picked up when ReadDir is created
    }
}

fn find_conflicts(src: &Path, dst: &Path) -> io::Result<Vec<DirEntry>> {
    let src_iter = fs::read_dir(src)?;
    let conflicts: Vec<DirEntry> = src_iter
        .filter_map(|entry_result| check_entry_for_conflict(entry_result, dst))
        .collect();

    Ok(conflicts)
}

pub(crate) async fn fetch_vlayer_files(dst: &Path) -> Result<(), CLIError> {
    let response = get(CONTRACTS_URL)
        .await
        .map_err(map_reqwest_error)?
        .bytes()
        .await
        .map_err(map_reqwest_error)?;

    let cursor = Cursor::new(response);

    let d = GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(d);

    let temp_dir = tempfile::tempdir()?;
    archive.zunpack(temp_dir.path())?;

    // vlayer contracts are in a directory called "vlayer"
    let downloaded_contracts = temp_dir.path().join("vlayer");

    let conflicts: Vec<DirEntry> = find_conflicts(&downloaded_contracts, dst)?;

    if !conflicts.is_empty() {
        todo!();
    }
    todo!();

    Ok(())
}

pub(crate) async fn fetch_vlayer_files_override(dst: &Path) -> Result<(), CLIError> {
    // overrides all files in the vlayer directory
    let response = get(CONTRACTS_URL)
        .await
        .map_err(map_reqwest_error)?
        .bytes()
        .await
        .map_err(map_reqwest_error)?;

    let cursor = Cursor::new(response);

    let d = GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(d);
    archive.zunpack(dst)?;

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
    use std::env::temp_dir;

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
        let temp_dir = tempfile::tempdir().unwrap();

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

    #[tokio::test]
    #[ignore]
    async fn test_get_vlayer_files_override() {
        // this test is ignored because it downloads a file from the internet
        // it's more of a sanity check for local tests, will be removed eventually
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_dir_path = temp_dir.path();
        fetch_vlayer_files_override(temp_dir_path).await.unwrap();

        // check that temp_dir_path/vlayer contains 1 file: "Simple.v.sol"
        let vlayer_dir = temp_dir_path.join("vlayer");
        let files = fs::read_dir(vlayer_dir).unwrap();
        let files: Vec<_> = files.map(|f| f.unwrap().file_name()).collect();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], "Simple.v.sol");
    }

    #[test]
    fn test_find_conflicts() {
        let temp_dir = tempfile::tempdir().unwrap();
        let src_path = temp_dir.path().join("src");
        let dst_path = temp_dir.path().join("dst");
        std::fs::create_dir_all(&src_path).unwrap();
        std::fs::create_dir_all(&dst_path).unwrap();

        let conflicts = find_conflicts(&src_path, &dst_path).unwrap();
        assert_eq!(conflicts.len(), 0);

        let file = src_path.join("file");
        std::fs::File::create(&file).unwrap();
        let conflicts = find_conflicts(&src_path, &dst_path).unwrap();
        assert_eq!(conflicts.len(), 0);

        let file = dst_path.join("file");
        std::fs::File::create(&file).unwrap();
        let conflicts = find_conflicts(&src_path, &dst_path).unwrap();
        assert_eq!(conflicts.len(), 1);
    }
}
