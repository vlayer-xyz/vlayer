use std::{
    fs,
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

use crate::errors::CLIError;

pub(crate) fn find_foundry_root(start: &Path) -> Result<PathBuf, CLIError> {
    let start = start.canonicalize()?;
    let git_root = find_git_root(&start)?;
    do_find_foundry_root_from(&start, &git_root)
}

pub(crate) fn copy_dir_to(src_dir: &Path, dst_dir: &Path) -> std::io::Result<()> {
    if !dst_dir.is_dir() {
        fs::create_dir_all(dst_dir)?;
    }

    for entry_result in src_dir.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        if file_type.is_dir() {
            copy_dir_to(&entry.path(), &dst_dir.join(file_name))?;
        } else if file_type.is_symlink() {
            let link = fs::read_link(entry.path())?;
            unix_fs::symlink(link, dst_dir.join(file_name))?;
        } else {
            fs::copy(entry.path(), dst_dir.join(file_name))?;
        }
    }

    Ok(())
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
        return Err(CLIError::GitError(String::from_utf8_lossy(&output.stderr).to_string()));
    }

    let path = from_utf8(&output.stdout)?.trim_end_matches('\n');
    Ok(PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempfile::tempdir;

    use super::*;
    use crate::test_utils::create_temp_git_repo;

    macro_rules! create_temp_dirs {
        ($src_dir:ident, $dst_dir:ident) => {
            let temp_dir = tempdir()?.into_path();
            let $src_dir = temp_dir.join("src");
            let $dst_dir = temp_dir.join("dst");

            std::fs::create_dir(&$src_dir).unwrap();
        };
    }

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
        let temp_dir = tempdir().unwrap();
        let result = find_git_root(temp_dir.path());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_git_root_nonexistent_path() {
        let temp_dir = tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("not_a_real_dir");
        let result = find_git_root(nonexistent_path);

        assert!(matches!(
            result.unwrap_err(),
            CLIError::CommandExecutionError(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn test_find_foundry_root_from_no_git() {
        let temp_dir = tempdir().unwrap();
        let result = find_foundry_root(temp_dir.path());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_foundry_root_from_no_foundry() {
        let temp_dir = create_temp_git_repo();
        let result = find_foundry_root(temp_dir.path());
        assert!(matches!(result.unwrap_err(), CLIError::NoFoundryError));
    }

    #[test]
    fn test_find_foundry_root_from() {
        let temp_dir = create_temp_git_repo();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();
        let result = find_foundry_root(temp_dir.path());
        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_root_from_subdir() {
        let temp_dir = create_temp_git_repo();
        let sub_dir = temp_dir.path().join("dir1").join("dir2");
        std::fs::create_dir_all(&sub_dir).unwrap();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();
        let result = find_foundry_root(&sub_dir);
        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_root_from_nonexistent_path() {
        let temp_dir = tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("not_a_real_dir");
        let result = find_foundry_root(&nonexistent_path);
        assert!(matches!(
            result.unwrap_err(),
            CLIError::CommandExecutionError(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }

    mod copy_dir_to {

        use super::*;

        #[test]
        fn copies_all_files_from_src_to_dest() -> Result<()> {
            create_temp_dirs!(src_dir, dst_dir);

            std::fs::write(src_dir.join("file1"), "file1").unwrap();
            std::fs::write(src_dir.join("file2"), "file2").unwrap();

            copy_dir_to(&src_dir, &dst_dir).unwrap();

            assert!(dst_dir.exists());
            assert!(dst_dir.join("file1").exists());
            assert!(dst_dir.join("file2").exists());

            Ok(())
        }

        #[test]
        fn handles_tangling_symlinks() -> anyhow::Result<()> {
            create_temp_dirs!(src_dir, dst_dir);

            let non_existent_file_path = PathBuf::from("/non/existent/file");
            let dst_tangling_symlink_path = dst_dir.join("tangling-symlink");
            let tangling_symlink_path = src_dir.join("tangling-symlink");

            unix_fs::symlink(non_existent_file_path.clone(), tangling_symlink_path)?;

            copy_dir_to(&src_dir, &dst_dir)?;

            assert!(!non_existent_file_path.exists());
            // Path::try_exists returns Ok(false) for broken symlinks
            assert!(!dst_tangling_symlink_path.try_exists()?);
            assert!(dst_tangling_symlink_path.is_symlink());

            assert_eq!(std::fs::read_link(dst_tangling_symlink_path)?, non_existent_file_path);

            Ok(())
        }
    }
}
