use clap::{Parser, Subcommand};
use server::server::serve;
use std::path::{Path, PathBuf};

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

/// https://github.com/foundry-rs/foundry/blob/fbd225194dff17352ba740cb3d6f2ad082030dd1/crates/config/src/utils.rs
/// Returns the path of the top-level directory of the working git tree. If there is no working
/// tree, an error is returned.
pub fn find_git_root_path(relative_to: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
    let path = relative_to.as_ref();
    let path = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(path)
        .output()
        .map_err(|e| anyhow::anyhow!("Failed to run git: {}", e))?
        .stdout;

    let path = std::str::from_utf8(&path)?.trim_end_matches('\n');
    Ok(PathBuf::from(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_examples_path() -> PathBuf {
        let cwd = &std::env::current_dir().unwrap();
        let mut examples_path = PathBuf::from(cwd.parent().unwrap().parent().unwrap());
        examples_path.push("examples/simple");
        examples_path
    }

    #[test]
    fn test_find_git_root_path() {
        // searches from current directory
        let path = Path::new(".");
        let result = find_git_root_path(path);
        let root_path = result.unwrap();
        assert!(root_path.is_dir());
        assert!(root_path.ends_with("vlayer"));
        println!("Git root path: {}", root_path.display());
    }

    #[test]
    fn test_find_git_root_path_from_examples() {
        // searches from examples directory
        let path = get_examples_path();
        let result = find_git_root_path(path);
        let root_path = result.unwrap();
        assert!(root_path.is_dir());
        assert!(root_path.ends_with("vlayer"));
        println!("Git root path: {}", root_path.display());
    }
}
