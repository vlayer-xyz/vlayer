use crate::errors::CLIError;
use crate::misc::path::find_git_root_path;
use clap::{Parser, Subcommand};
use server::server::serve;
use std::path::PathBuf;

pub mod errors;
pub mod misc;

#[cfg(test)]
mod test_utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Serve,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve => {
            println!("running 'vlayer serve'");
            serve().await
        }
        Commands::Init => {
            println!("running 'vlayer init'");
            Ok(())
        }
    }
}

pub fn find_foundry_project_root_path(start: &PathBuf) -> Result<PathBuf, CLIError> {
    let git_root = find_git_root_path(start)?;

    let mut current = start.as_path().canonicalize()?;

    // traverse as long as we're in the current git repository
    while current.starts_with(&git_root) {
        let file_path = current.join("foundry.toml");
        if file_path.is_file() {
            return Ok(current.to_path_buf());
        }
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            break;
        }
    }
    Err(CLIError::NoFoundryError)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_temp_git_repo;

    #[test]
    fn test_find_foundry_project_root_path_no_git() {
        let temp_dir = tempfile::tempdir().unwrap();
        let result = find_foundry_project_root_path(&temp_dir.path().to_path_buf());

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();

        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_foundry_project_root_path_no_foundry() {
        let temp_dir = create_temp_git_repo();
        let result = find_foundry_project_root_path(&temp_dir.path().to_path_buf());
        assert!(matches!(result.unwrap_err(), CLIError::NoFoundryError));
    }

    #[test]
    fn test_find_foundry_project_root_path() {
        let temp_dir = create_temp_git_repo();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();
        let result = find_foundry_project_root_path(&temp_dir.path().to_path_buf());

        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_project_root_path_subdir() {
        let temp_dir = create_temp_git_repo();
        let sub_dir1 = temp_dir.path().join("dir1");
        let sub_dir2 = sub_dir1.join("dir2");
        std::fs::create_dir_all(&sub_dir2).unwrap();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();
        let result = find_foundry_project_root_path(&sub_dir2);

        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_project_root_path_nonexistent_path() {
        let temp_dir = tempfile::tempdir().unwrap();
        let nonexistent_path = temp_dir.path().join("not_a_real_dir");
        let result = find_foundry_project_root_path(&nonexistent_path);
        assert!(matches!(
            result.unwrap_err(),
            CLIError::CommandExecutionError(err) if err.kind() == std::io::ErrorKind::NotFound
        ));
    }
}
