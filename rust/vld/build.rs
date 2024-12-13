use std::{
    env, fs,
    fs::File,
    io,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("directories.rs");

    _remove_file_if_exists(&dest_path).unwrap();
    let mut f = fs::File::create(dest_path).unwrap();
    writeln!(f, "use clap::Subcommand;\n").unwrap();

    _add_enum(&mut f, "Examples", _get_dir_list("../../examples"));
    _add_enum(&mut f, "Rust", _get_dir_list("../"));
    _add_enum(&mut f, "JS", _get_dir_list("../../packages"));
}

fn _camel_case(dir: &str) -> String {
    dir.split(|c| c == '_' || c == '-')
        .map(|s| {
            s.chars()
                .enumerate()
                .map(|(i, c)| if i == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>()
        })
        .collect()
}

fn _get_dir_list(relative_path: &str) -> Vec<String> {
    let scan_dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join(relative_path);

    fs::read_dir(scan_dir)
        .unwrap()
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_dir() {
                path.file_name()?.to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn _add_enum(file: &mut File, name: &str, commands: Vec<String>) {
    writeln!(
        file,
        "#[derive(Subcommand, Debug)]
pub enum {name} {{"
    )
    .unwrap();

    commands.iter().for_each(|dir| {
        writeln!(file, "\t{},", _camel_case(dir)).unwrap();
    });
    writeln!(file, "}}\n").unwrap();

    let formats = commands
        .iter()
        .map(|dir| {
            format!(
                "{name}::{camel} => \"{dir}\".into(),",
                name = name,
                camel = _camel_case(dir),
                dir = dir
            )
        })
        .collect::<Vec<String>>()
        .join("\n\t\t\t");

    writeln!(
        file,
        "impl ToString for {name} {{
    fn to_string(&self) -> String {{
        match self {{
            {formats}
        }}
    }}
}}
"
    )
    .unwrap()
}

fn _remove_file_if_exists(path: &PathBuf) -> Result<(), io::Error> {
    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e),
    }
}
