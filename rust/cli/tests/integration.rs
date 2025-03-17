#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_cli_output() {
        let mut cmd = Command::cargo_bin("vlayer").unwrap();
        cmd.arg("web-proof-fetch").assert().success();
    }
}
