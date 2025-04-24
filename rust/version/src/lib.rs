pub fn version() -> String {
    if env!("VLAYER_RELEASE") == "stable" {
        return env!("CARGO_PKG_VERSION").to_string();
    }
    let build_date = env!("VERGEN_BUILD_DATE").replace("-", "");
    [
        env!("CARGO_PKG_VERSION"),
        env!("VLAYER_RELEASE"),
        build_date.as_str(),
        env!("VERGEN_GIT_SHA"),
    ]
    .join("-")
}

pub fn is_stable() -> bool {
    env!("VLAYER_RELEASE") == "stable"
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn has_pkg_version() {
        let version = version();
        let pkg_version = version.split('-').next().unwrap();

        assert_eq!(env!("CARGO_PKG_VERSION"), pkg_version);
    }

    #[test]
    fn has_build_mode() {
        let version = version();
        let build_mode = version.split('-').nth(1).unwrap();

        assert_eq!("dev", env!("VLAYER_RELEASE"));
        assert_eq!(env!("VLAYER_RELEASE"), build_mode);
    }

    #[test]
    fn has_build_date() {
        // "yyyymmdd"
        let date_regex = Regex::new(r"[0-9]{4}[0-1][0-9][0-3][0-9]").unwrap();

        let version = version();
        let build_date = version.split('-').nth(2).unwrap();

        assert!(date_regex.is_match(build_date));
    }

    #[test]
    fn has_build_commit() {
        // ex "dd603d4c45d469a6e46bb47ea0677c0a5d757a4d" - default result of `git rev-parse` has length of 40
        let commit_regex = Regex::new(r"[0-9a-f]{40}").unwrap();

        let version = version();
        let build_commit = version.split('-').nth(3).unwrap();

        assert!(commit_regex.is_match(build_commit));
    }
}
