#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn test_web_proof_fetch_requires_url() {
        let mut cmd = Command::cargo_bin("vlayer").unwrap();
        cmd.args(["web-proof-fetch"])
            .assert()
            .failure()
            .code(2)
            .stderr(predicates::str::contains(
                "error: the following required arguments were not provided:\n  --url <URL>",
            ))
            .stdout(predicates::str::is_empty());
    }
}
