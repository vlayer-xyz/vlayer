use crate::errors::CLIError;

pub(crate) fn get_src_from_string(contents: String) -> Result<String, CLIError> {
    let config = toml::from_str(&contents).map_err(|e| CLIError::TomlError(e.to_string()))?;

    let result = get_src_from_toml(config);
    match result {
        Some(src) => Ok(src),
        None => Err(CLIError::TomlError(
            "No source found in foundry.toml".to_string(),
        )),
    }
}

fn get_src_from_toml(config: toml::Table) -> Option<String> {
    let default_src = config
        .get("profile")?
        .get("default")?
        .get("src")?
        .as_str()?;
    Some(default_src.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_src_from_string() {
        let contents = r#"
            [profile.default]
            src = "src"
        "#;
        let result = get_src_from_string(contents.to_string());
        assert_eq!(result.unwrap(), "src".to_string());
    }

    #[test]
    fn test_get_src_from_string_fail() {
        let contents = r#"
            [profile.dev]
            src = "src"
        "#;
        let result = get_src_from_string(contents.to_string());
        assert!(
            matches!(result.unwrap_err(), CLIError::TomlError(errormsg) if errormsg == "No source found in foundry.toml")
        );
    }

    #[test]
    fn test_get_src_from_string_invalid_toml() {
        let invalid_toml = r#"
            [profile.default]
            src = "src"aaa
        "#;
        let result = get_src_from_string(invalid_toml.to_string());
        assert!(
            matches!(result.unwrap_err(), CLIError::TomlError(errormsg) if errormsg != "No source found in foundry.toml")
        ); // A long error message is returned, specifying the error in the TOML
    }
}
