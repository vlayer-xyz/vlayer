use std::{
    fs,
    os::unix::fs as unix_fs,
    path::{Path, PathBuf},
    process::Command,
    str::from_utf8,
};

use anyhow::Context;

use crate::errors::{Error, Result};

pub(crate) fn find_foundry_root(start: &Path) -> Result<PathBuf> {
    let start = start.canonicalize()?;
    let git_root = find_git_root(&start)?;
    do_find_foundry_root_from(&start, &git_root)
}

const IGNORES: &[&str] = &["node_modules"];

pub(crate) fn copy_dir_to(src_dir: &Path, dst_dir: &Path) -> anyhow::Result<()> {
    copy_dir_to_dir_with_ignores(src_dir, dst_dir, IGNORES)
}

fn is_ignored<P: AsRef<Path>>(path: &Path, ignores: &[P]) -> bool {
    ignores.iter().any(|x| path.ends_with(x))
}

pub(crate) fn copy_dir_to_dir_with_ignores<P: AsRef<Path>>(
    src_dir: &Path,
    dst_dir: &Path,
    ignores: &[P],
) -> anyhow::Result<()> {
    if !dst_dir.is_dir() {
        fs::create_dir_all(dst_dir)
            .with_context(|| format!("Failed to create path '{}'", dst_dir.display()))?;
    }

    for entry_result in src_dir
        .read_dir()
        .with_context(|| format!("Failed to open directory '{}' for reading", src_dir.display()))?
    {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let file_name = entry.file_name();
        let dst = dst_dir.join(file_name);
        let src = entry.path();

        if is_ignored(&src, ignores) {
            continue;
        }

        if file_type.is_dir() {
            copy_dir_to_dir_with_ignores(&src, &dst, ignores).with_context(|| {
                format!("Failed to copy directory from '{}' to '{}'", src.display(), dst.display())
            })?;
        } else if file_type.is_symlink() {
            let link = fs::read_link(&src)?;
            let mut full_link_path = src.parent().map_or("/".into(), PathBuf::from);
            full_link_path.push(&link);
            let orig = fs::canonicalize(&full_link_path).with_context(|| {
                format!("Failed to canonicalize path '{}'", full_link_path.display())
            })?;
            tracing::info!("Symlinking {} -> {}", dst.display(), orig.display());
            unix_fs::symlink(&orig, &dst).with_context(|| {
                format!("Failed to symlink '{}' as '{}'", orig.display(), dst.display())
            })?;
        } else {
            fs::copy(&src, &dst).with_context(|| {
                format!("Failed to copy file from '{}' to '{}'", src.display(), dst.display())
            })?;
        }
    }

    Ok(())
}

fn do_find_foundry_root_from(current: &Path, git_root: &PathBuf) -> Result<PathBuf> {
    if !current.starts_with(git_root) {
        return Err(Error::NoFoundry);
    }

    let path = current.join("foundry.toml");

    if path.is_file() {
        return Ok(current.to_path_buf());
    }

    if let Some(parent) = current.parent() {
        do_find_foundry_root_from(parent, git_root)
    } else {
        Err(Error::NoFoundry)
    }
}

/// https://github.com/foundry-rs/foundry/blob/fbd225194dff17352ba740cb3d6f2ad082030dd1/crates/config/src/utils.rs
pub fn find_git_root(relative_to: impl AsRef<Path>) -> Result<PathBuf> {
    let path = relative_to.as_ref();
    let path = &path.canonicalize()?;
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .with_context(|| {
            format!(
                "Invorking 'git rev-parse --show-toplevel' from directory {} failed",
                path.display()
            )
        })?;

    if !output.status.success() {
        return Err(Error::Git(String::from_utf8_lossy(&output.stderr).to_string()));
    }

    let path = from_utf8(&output.stdout)?.trim_end_matches('\n');
    Ok(PathBuf::from(path))
}

pub fn find_file_up_tree(name: &str) -> anyhow::Result<Option<PathBuf>> {
    let mut path = std::env::current_dir()?;
    loop {
        path.push(name);
        if path.exists() {
            return Ok(Some(path));
        }
        path.pop();
        if !path.pop() {
            return Ok(None);
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    use crate::test_utils::create_temp_git_repo;

    macro_rules! create_temp_dirs {
        ($src_dir:ident, $dst_dir:ident) => {
            let temp_dir = tempdir().unwrap().into_path();
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
        assert!(matches!(error, Error::Git(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_git_root_nonexistent_path() {
        let temp_dir = tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("not_a_real_dir");
        let result = find_git_root(nonexistent_path);

        assert!(matches!(
            result.unwrap_err(),
            Error::CommandExecution(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }

    #[test]
    fn test_find_foundry_root_from_no_git() {
        let temp_dir = tempdir().unwrap();
        let result = find_foundry_root(temp_dir.path());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, Error::Git(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_foundry_root_from_no_foundry() {
        let temp_dir = create_temp_git_repo();
        let result = find_foundry_root(temp_dir.path());
        assert!(matches!(result.unwrap_err(), Error::NoFoundry));
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
            Error::CommandExecution(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }

    mod copy_dir_to {

        use super::*;

        #[test]
        fn copies_all_files_from_src_to_dest() {
            create_temp_dirs!(src_dir, dst_dir);

            fs::write(src_dir.join("file1"), "file1").unwrap();
            fs::write(src_dir.join("file2"), "file2").unwrap();

            copy_dir_to(&src_dir, &dst_dir).unwrap();

            assert!(dst_dir.exists());
            assert!(dst_dir.join("file1").exists());
            assert!(dst_dir.join("file2").exists());
        }

        #[test]
        fn dangling_symlinks_should_fail() {
            create_temp_dirs!(src_dir, dst_dir);

            let non_existent_file_path = src_dir.join("non/existent/file");
            let dangling_symlink_path = src_dir.join("dangling-symlink");

            unix_fs::symlink(non_existent_file_path.clone(), dangling_symlink_path).unwrap();

            assert!(!non_existent_file_path.exists());
            assert!(copy_dir_to(&src_dir, &dst_dir).is_err());
        }

        #[test]
        fn correctly_resolve_symlinks() {
            create_temp_dirs!(src_dir, dst_dir);

            let target = src_dir.join("target");
            fs::write(&target, "dummy").unwrap();
            let link = src_dir.join("link");
            unix_fs::symlink(&target, link).unwrap();

            copy_dir_to(&src_dir, &dst_dir).unwrap();

            let dst_link = dst_dir.join("link");
            assert!(dst_link.is_symlink());
            // On macOS, /var/tmp may be a symlink to /private/var/tmp, so we fuzzy match.
            assert!(
                fs::read_link(&dst_link)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains(target.to_str().unwrap())
            );
            assert_eq!("dummy", fs::read_to_string(dst_link).unwrap());
        }

        #[test]
        fn respect_ignores() {
            create_temp_dirs!(src_dir, dst_dir);

            fs::create_dir_all(src_dir.join("outer/ignored/inner")).unwrap();
            fs::create_dir_all(src_dir.join("outer/inner")).unwrap();
            fs::create_dir_all(src_dir.join("ignored")).unwrap();

            copy_dir_to_dir_with_ignores(&src_dir, &dst_dir, &["ignored"]).unwrap();

            assert!(dst_dir.join("outer").exists());
            assert!(dst_dir.join("outer/inner").exists());
            assert!(!dst_dir.join("outer/ignored/inner").exists());
            assert!(!dst_dir.join("outer/ignored").exists());
            assert!(!dst_dir.join("ignored").exists());
        }
    }
}
