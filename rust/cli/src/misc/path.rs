use crate::errors::CLIError;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::from_utf8;

pub(crate) fn find_foundry_root() -> Result<PathBuf, CLIError> {
    let current_dir = std::env::current_dir()?;
    find_foundry_root_from(&current_dir)
}

fn find_foundry_root_from(start: &Path) -> Result<PathBuf, CLIError> {
    let start = start.canonicalize()?;
    let git_root = find_git_root(&start)?;
    do_find_foundry_root_from(&start, &git_root)
}

fn do_find_foundry_root_from(current: &Path, git_root: &PathBuf) -> Result<PathBuf, CLIError> {
    if !current.starts_with(git_root) {
        return Err(CLIError::NoFoundryError);
    }

    let path = current.join("foundry.toml");

    if path.is_file() {
        return Ok(current.to_path_buf());
    }

    if let Some(parent) = current.parent() {
        do_find_foundry_root_from(parent, git_root)
    } else {
        Err(CLIError::NoFoundryError)
    }
}

/// https://github.com/foundry-rs/foundry/blob/fbd225194dff17352ba740cb3d6f2ad082030dd1/crates/config/src/utils.rs
pub fn find_git_root(relative_to: impl AsRef<Path>) -> Result<PathBuf, CLIError> {
    let path = relative_to.as_ref();
    let path = &path.canonicalize()?;
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
    use crate::test_utils::create_temp_git_repo;

    #[test]
    fn test_find_git_root() {
        let temp_dir = create_temp_git_repo();
        let result = find_git_root(temp_dir.path()).unwrap();
        let expected = temp_dir.path().canonicalize().unwrap();
        assert!(result.is_dir());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_git_root_deep() {
        let temp_dir = create_temp_git_repo();
        let git_root_path = temp_dir.path().canonicalize().unwrap();
        let sub_dir1 = temp_dir.path().join("dir1");
        let sub_dir2 = sub_dir1.join("dir2");
        std::fs::create_dir_all(&sub_dir2).unwrap();
        let root_path1 = find_git_root(&sub_dir1).unwrap();
        let root_path2 = find_git_root(&sub_dir2).unwrap();
        assert!(root_path1.is_dir());
        assert!(root_path2.is_dir());
        assert_eq!(root_path1, root_path2);
        assert_eq!(git_root_path, root_path1);
    }

    #[test]
    fn test_find_git_root_fail() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = find_git_root(temp_dir.path());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_git_root_nonexistent_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("not_a_real_dir");
        let result = find_git_root(nonexistent_path);

        assert!(matches!(
            result.unwrap_err(),
            CLIError::CommandExecutionError(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn test_find_foundry_root_from_no_git() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = find_foundry_root_from(temp_dir.path());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_foundry_root_from_no_foundry() {
        let temp_dir = create_temp_git_repo();
        let result = find_foundry_root_from(temp_dir.path());
        assert!(matches!(result.unwrap_err(), CLIError::NoFoundryError));
    }

    #[test]
    fn test_find_foundry_root_from() {
        let temp_dir = create_temp_git_repo();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();
        let result = find_foundry_root_from(temp_dir.path());
        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_root_from_subdir() {
        let temp_dir = create_temp_git_repo();
        let sub_dir = temp_dir.path().join("dir1").join("dir2");
        std::fs::create_dir_all(&sub_dir).unwrap();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();
        let result = find_foundry_root_from(&sub_dir);
        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_root_from_nonexistent_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("not_a_real_dir");
        let result = find_foundry_root_from(&nonexistent_path);
        assert!(matches!(
            result.unwrap_err(),
            CLIError::CommandExecutionError(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }
}
