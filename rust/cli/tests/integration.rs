#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::str::contains;

    #[test]
    fn test_web_proof_fetch_requires_url() {
        let mut cmd = Command::cargo_bin("vlayer").unwrap();
        cmd.args(["web-proof-fetch"])
            .assert()
            .failure()
            .code(2)
            .stderr(contains(
                "error: the following required arguments were not provided:\n  --url <URL>",
            ))
            .stdout(predicates::str::is_empty());
    }

    #[test]
    fn test_all_logs_go_to_stderr() {
        let mut cmd = Command::cargo_bin("vlayer").unwrap();
        cmd.env("RUST_LOG", "trace")
            .args(["test-logging-configuration"])
            .assert()
            .stderr(contains("error"))
            .stderr(contains("warn"))
            .stderr(contains("info"))
            .stderr(contains("debug"))
            .stderr(contains("trace"))
            .stderr(contains("printed").count(0))
            .stdout("printed\n");
    }
    #[test]
    fn test_logs_cut_out_at_info_by_default() {
        let mut cmd = Command::cargo_bin("vlayer").unwrap();
        cmd.env("RUST_LOG", "")
            .args(["test-logging-configuration"])
            .assert()
            .stderr(contains("error"))
            .stderr(contains("warn"))
            .stderr(contains("info"))
            .stderr(contains("debug").count(0))
            .stderr(contains("trace").count(0))
            .stderr(contains("printed").count(0));
    }
}
