#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    #[ignore]
    fn test_cli_output() {
        let mut cmd = Command::cargo_bin("vlayer").unwrap();
        cmd.args([
            "web-proof-fetch",
            "--url",
            "https://lotr-api.online:3011/regular_json?are_you_sure=yes&auth=s3cret_t0ken",
            "--host",
            "127.0.0.1",
        ])
        .assert()
        .success();
    }
}
