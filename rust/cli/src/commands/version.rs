use clap::builder::{IntoResettable, Resettable, Str};

pub struct Version;

impl IntoResettable<Str> for Version {
    fn into_resettable(self) -> Resettable<Str> {
        version().into_resettable()
    }
}

pub(crate) fn version() -> String {
    let build_date = env!("VERGEN_BUILD_DATE");
    let build_date_without_dashes = build_date.replace('-', "");

    [
        env!("CARGO_PKG_VERSION"),
        env!("VLAYER_RELEASE"),
        build_date_without_dashes.as_str(),
        env!("VERGEN_GIT_SHA"),
    ]
    .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod version_msg {
        use super::*;

        #[test]
        fn contains_version_line() {
            assert_eq!(version().lines().next().unwrap(), version());
        }
    }

    mod version {
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
            // ex "14d8bb4" - default result of `git rev-parse --short` has length of 7
            let commit_regex = Regex::new(r"[0-9a-f]{7}").unwrap();

            let version = version();
            let build_commit = version.split('-').nth(3).unwrap();

            assert!(commit_regex.is_match(build_commit));
        }
    }
}
