use crate::errors::CLIError;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

/// https://github.com/foundry-rs/foundry/blob/fbd225194dff17352ba740cb3d6f2ad082030dd1/crates/config/src/utils.rs
pub fn find_git_root_path(relative_to: impl AsRef<Path>) -> Result<PathBuf, CLIError> {
    let path = relative_to.as_ref();
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()?;

    if !output.status.success() {
        return Err(CLIError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let path = from_utf8(&output.stdout)?.trim_end_matches('\n');
    Ok(PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_temp_repo;

    #[test]
    fn test_find_git_root_path() {
        let temp_dir = create_temp_repo();
        let result = find_git_root_path(temp_dir.path());
        let root_path = result.unwrap();
        assert!(root_path.is_dir());
    }

    #[test]
    fn test_find_git_root_path_deep() {
        let temp_dir = create_temp_repo();
        let sub_dir1 = temp_dir.path().join("dir1");
        let sub_dir2 = sub_dir1.join("dir2");
        std::fs::create_dir_all(&sub_dir2).unwrap();
        let root_path1 = find_git_root_path(&sub_dir1).unwrap();
        let root_path2 = find_git_root_path(&sub_dir2).unwrap();
        assert!(root_path1.is_dir());
        assert!(root_path2.is_dir());
        assert_eq!(root_path1, root_path2);
    }

    #[test]
    fn test_find_git_root_path_fail() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = find_git_root_path(temp_dir.path());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }
}
