use clap::builder::{IntoResettable, Resettable, Str};

use call_guest_wrapper::RISC0_CALL_GUEST_ID;

pub struct Version;

impl IntoResettable<Str> for Version {
    fn into_resettable(self) -> Resettable<Str> {
        version_msg().into_resettable()
    }
}

fn version_msg() -> String {
    [version(), call_guest_id()].join("\n")
}

fn call_guest_id() -> String {
    format!("CALL_GUEST_ID: {}", to_hex(RISC0_CALL_GUEST_ID.into()))
}

fn version() -> String {
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

fn to_hex(arr: Vec<u32>) -> String {
    arr.into_iter()
        .map(|e| format!("{e:08x}"))
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    mod version_msg {
        use super::*;

        #[test]
        fn contains_version_line() {
            let version_msg = version_msg();
            let second_line = version_msg.lines().next().unwrap();

            assert_eq!(second_line, version());
        }

        #[test]
        fn contains_guest_id() {
            let version = version_msg();
            let second_line = version.lines().nth(1).unwrap();

            assert_eq!(second_line, call_guest_id());
        }
    }

    mod guest_id {
        use super::*;

        #[test]
        fn guest_id_equals_to_compiled_in_version() {
            let guest_id_line = call_guest_id();
            let guest_id = guest_id_line.trim_start_matches("CALL_GUEST_ID: ");

            assert_eq!(guest_id, to_hex(RISC0_CALL_GUEST_ID.into()));
        }
    }

    mod version {
        use super::*;

        use regex::Regex;

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

    mod to_hex {
        use super::*;

        #[test]
        fn returns_hex_repr_of_an_array() {
            assert_eq!("ffffffff00000000", to_hex(vec![u32::MAX, u32::MIN]));
        }

        #[test]
        fn does_not_cut_leading_zeros() {
            assert_eq!("00000001", to_hex(vec![1]));
        }
    }
}
