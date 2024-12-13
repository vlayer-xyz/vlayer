use std::{env, fs, io, io::Write, path::PathBuf, process::Command};

use rustyline::{error::ReadlineError, DefaultEditor};

const BASH: &str = r#"vladcd () {
    local result=$(vlad "$@")
    if [ $? -eq 0 ]; then
        cd "$result"
    fi
}
"#;

fn vladcd_exists() -> bool {
    which::which("vladcd").is_ok()
}

fn update_zhrc() -> Result<(), io::Error> {
    if vladcd_exists() {
        println!("Skipping .zshrc update");
        return Ok(());
    }
    let mut shell_rc_path = env::home_dir().unwrap();
    shell_rc_path.push(".zshrc");
    let mut shell_rc = fs::OpenOptions::new().append(true).open(shell_rc_path)?;

    println!("Run `source ~/.zshrc` to update your shell");
    writeln!(shell_rc, "{BASH}")
}

fn find_file_up_tree(name: &str, substring: &str) -> Result<PathBuf, io::Error> {
    let mut path = env::current_dir()?;
    loop {
        path.push(name);
        if path.exists() && fs::read_to_string(&path)?.contains(substring) {
            path.pop();
            return Ok(path);
        }
        path.pop();
        if !path.pop() {
            return Ok(env::current_dir()?);
        }
    }
}

fn write_config_file(path: &str) -> Result<(), io::Error> {
    let mut config_path = env::home_dir().unwrap();
    config_path.push(".vlad");

    let mut file = fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open(config_path)?;
    writeln!(file, "VLAYER_PATH={path}")
}

fn try_find_vlayer_dir() {
    let default_value = find_file_up_tree("LICENSE", "Copyright (c) 2024 vlayer").unwrap();
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline_with_initial(
            "Provide vlayer root directory: ",
            (&default_value.display().to_string(), ""),
        ) {
            Ok(path) if fs::exists(&path).unwrap_or(false) => {
                write_config_file(&path).unwrap();
                return;
            }
            Ok(path) => {
                println!("{path} doesn't exist");
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            _ => {}
        }
    }
}
pub fn init() {
    update_zhrc().unwrap();
    try_find_vlayer_dir();
}
