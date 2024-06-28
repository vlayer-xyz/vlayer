use std::process::Command;

pub fn create_temp_git_repo() -> tempfile::TempDir {
    // Create a new temporary directory
    let temp_dir = tempfile::tempdir().unwrap();

    // Initialize a new Git repository in the temp directory
    let repo_path = temp_dir.path();
    Command::new("git")
        .arg("init")
        .arg(repo_path)
        .output()
        .unwrap();

    let file_path = repo_path.join("test_file.txt");
    std::fs::File::create(file_path).unwrap();

    temp_dir
}
