use crate::errors::CLIError;
use crate::misc::path::find_git_root;
use clap::{Parser, Subcommand};
use server::server::serve;
use std::path::PathBuf;

#[cfg(test)]
use std::path::Path;

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
async fn main() {
    match run().await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

async fn run() -> Result<(), CLIError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Serve => {
            println!("running 'vlayer serve'");
            serve().await?;
        }
        Commands::Init => {
            println!(
                "running 'vlayer init' from directory: {:?}",
                std::env::current_dir()?
            );
            let root = find_foundry_root()?;
            println!("foundry root: {:?}", root);
        }
    }
    Ok(())
}

fn find_foundry_root() -> Result<PathBuf, CLIError> {
    let current_dir = std::env::current_dir()?;
    find_foundry_root_from(&current_dir)
}

fn find_foundry_root_from(start: &PathBuf) -> Result<PathBuf, CLIError> {
    let git_root = find_git_root(start)?;

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

    fn change_directory(new_dir: &Path) -> std::io::Result<()> {
        std::env::set_current_dir(new_dir)?;
        Ok(())
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

    #[test]
    fn test_find_foundry_root() {
        let temp_dir = create_temp_git_repo();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();

        change_directory(temp_dir.path()).unwrap();
        let result = find_foundry_root();

        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }

    #[test]
    fn test_find_foundry_root_no_git() {
        let temp_dir = tempfile::tempdir().unwrap();

        change_directory(temp_dir.path()).unwrap();
        let result = find_foundry_root();

        let expected_error_msg =
            "fatal: not a git repository (or any of the parent directories): .git\n".to_string();
        let error = result.unwrap_err();
        assert!(matches!(error, CLIError::GitError(msg) if msg == expected_error_msg));
    }

    #[test]
    fn test_find_foundry_root_no_foundry() {
        let temp_dir = create_temp_git_repo();

        change_directory(temp_dir.path()).unwrap();
        let result = find_foundry_root();

        assert!(matches!(result.unwrap_err(), CLIError::NoFoundryError));
    }

    #[test]
    fn test_find_foundry_root_subdir() {
        let temp_dir = create_temp_git_repo();
        let sub_dir1 = temp_dir.path().join("dir1");
        let sub_dir2 = sub_dir1.join("dir2");
        std::fs::create_dir_all(&sub_dir2).unwrap();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::File::create(file_path).unwrap();

        change_directory(&sub_dir2).unwrap();
        let result = find_foundry_root();

        assert_eq!(result.unwrap(), temp_dir.path().canonicalize().unwrap());
    }
}
