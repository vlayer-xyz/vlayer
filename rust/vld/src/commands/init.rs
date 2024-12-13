use std::{env, fs, io, io::Write, path::PathBuf};

use rustyline::{error::ReadlineError, DefaultEditor};

const BASH: &str = r#"
vladcd () {
    local result=$(vlad "$@")
    if [ $? -eq 0 ]; then
        cd "$result"
    fi
}
"#;

fn vladcd_exists(path: &PathBuf) -> bool {
    fs::read_to_string(path).unwrap().contains("vladcd")
}

fn update_zhrc(path: String) -> Result<(), io::Error> {
    println!("{}", env!("OUT_DIR"));
    let mut shell_rc_path = env::home_dir().unwrap();
    shell_rc_path.push(".zshrc");

    if vladcd_exists(&shell_rc_path) {
        println!("Skipping .zshrc update");
        return Ok(());
    }

    let mut shell_rc = fs::OpenOptions::new().append(true).open(shell_rc_path)?;

    println!("Run `source ~/.zshrc` to update your shell");
    writeln!(shell_rc, "alias vlad=\"{}/rust/target/debug/vld\"", path)?;
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

fn try_find_vlayer_dir() -> Option<String> {
    let default_value = find_file_up_tree("LICENSE", "Copyright (c) 2024 vlayer").unwrap();
    let mut rl = DefaultEditor::new().unwrap();
    loop {
        match rl.readline_with_initial(
            "Provide vlayer root directory: ",
            (&default_value.display().to_string(), ""),
        ) {
            Ok(path) if fs::exists(&path).unwrap_or(false) => {
                write_config_file(&path).unwrap();
                return Some(path);
            }
            Ok(path) => {
                println!("{path} doesn't exist");
            }
            Err(ReadlineError::Interrupted) => {
                return None;
            }
            Err(ReadlineError::Eof) => {
                return None;
            }
            _ => {}
        }
    }
}
pub fn init() {
    let vlayer_dir = try_find_vlayer_dir();
    update_zhrc(vlayer_dir.unwrap()).unwrap();
}
