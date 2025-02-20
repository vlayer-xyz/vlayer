use std::path::Path;

use crate::errors::{Error, Result};

pub(crate) fn get_src_from_str(contents: impl AsRef<str>) -> Result<String> {
    let config = toml::from_str(contents.as_ref())?;

    let result = get_src_from_toml(&config);
    match result {
        Some(src) => Ok(src),
        None => Err(Error::NoSrcInFoundryToml),
    }
}

fn get_src_from_toml(config: &toml::Table) -> Option<String> {
    let default_src = config
        .get("profile")?
        .get("default")?
        .get("src")?
        .as_str()?;
    Some(default_src.to_string())
}

pub(crate) fn add_deps_to_foundry_toml(foundry_root: &Path) -> Result<()> {
    do_add_deps_to_foundry_toml(&foundry_root.join("foundry.toml"))
}

fn do_add_deps_to_foundry_toml(file_path: &Path) -> Result<()> {
    let mut foundry_toml = toml::from_str(&std::fs::read_to_string(file_path)?)?;

    add_deps_to_toml_table(&mut foundry_toml);
    let new_contents = foundry_toml.to_string();
    std::fs::write(file_path, new_contents)?;
    Ok(())
}

fn add_deps_to_toml_table(foundry_toml: &mut toml::Table) -> Option<()> {
    let dependencies: toml::Value = toml::Value::String("dependencies".to_string());

    let libs = foundry_toml
        .entry("profile")
        .or_insert(toml::Value::Table(toml::Table::new()))
        .as_table_mut()?
        .entry("default")
        .or_insert(toml::Value::Table(toml::Table::new()))
        .as_table_mut()?
        .entry("libs")
        .or_insert(toml::Value::Array(toml::value::Array::new()));

    if let toml::Value::Array(libs_inner) = libs {
        if !libs_inner.iter().any(|v| v == &dependencies) {
            libs_inner.push(dependencies);
        }
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_deps_to_foundry_toml() {
        let contents = r#"
            [profile.default]
            src = "src"
            libs = ["lib"]
        "#;
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("foundry.toml");
        std::fs::write(&file_path, contents).unwrap();

        do_add_deps_to_foundry_toml(&file_path).unwrap();

        let new_contents = std::fs::read_to_string(file_path).unwrap();

        let expected = "[profile.default]\nsrc = \"src\"\nlibs = [\"lib\", \"dependencies\"]\n";

        assert_eq!(new_contents, expected);
    }

    #[test]
    fn test_add_deps_to_toml_table_empty() {
        let mut foundry_toml = toml::value::Table::new();
        add_deps_to_toml_table(&mut foundry_toml);

        let expected = "[profile.default]\nlibs = [\"dependencies\"]\n";
        let expected = toml::from_str(expected).unwrap();

        assert_eq!(foundry_toml, expected);
    }

    #[test]
    fn test_add_deps_to_toml_table_empty_libs() {
        let contents = "[profile.default]\nlibs = []\n";
        let mut foundry_toml = toml::from_str(contents).unwrap();

        add_deps_to_toml_table(&mut foundry_toml);

        let expected = "[profile.default]\nlibs = [\"dependencies\"]\n";
        let expected = toml::from_str(expected).unwrap();

        assert_eq!(foundry_toml, expected);
    }

    #[test]
    fn test_add_deps_to_toml_table() {
        let contents = r#"
            [profile.default]
            src = "src"
            libs = ["lib1", "lib2"]
        "#;
        let mut foundry_toml = toml::from_str(contents).unwrap();
        add_deps_to_toml_table(&mut foundry_toml);

        let expected = r#"
            [profile.default]
            src = "src"
            libs = ["lib1", "lib2", "dependencies"]
        "#;
        let expected = toml::from_str(expected).unwrap();

        assert_eq!(foundry_toml, expected);
    }

    #[test]
    fn test_add_deps_to_toml_table_no_change() {
        let contents = r#"
            [profile.default]
            src = "src"
            libs = ["lib1", "dependencies", "lib2"]
        "#;
        let mut foundry_toml = toml::from_str(contents).unwrap();
        add_deps_to_toml_table(&mut foundry_toml);

        let expected = r#"
            [profile.default]
            src = "src"
            libs = ["lib1", "dependencies", "lib2"]
        "#;
        let expected = toml::from_str(expected).unwrap();

        assert_eq!(foundry_toml, expected);
    }

    #[test]
    fn test_get_src_from_string() {
        let contents = r#"
            [profile.default]
            src = "src"
        "#;
        let result = get_src_from_str(contents);
        assert_eq!(result.unwrap(), "src".to_string());
    }

    #[test]
    fn test_get_src_from_string_fail() {
        let contents = r#"
            [profile.dev]
            src = "src"
        "#;
        let result = get_src_from_str(contents);
        assert!(matches!(result.unwrap_err(), Error::NoSrcInFoundryToml));
    }

    #[test]
    fn test_get_src_from_string_invalid_toml() {
        let invalid_toml = r#"
            [profile.default]
            src = "src"aaa
        "#;
        let result = get_src_from_str(invalid_toml);
        assert!(matches!(result.unwrap_err(), Error::Toml(..)));
    }
}
