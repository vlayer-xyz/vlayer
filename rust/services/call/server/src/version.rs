use clap::builder::{IntoResettable, Resettable, Str};
use guest_wrapper::CALL_GUEST_ELF;
pub use version::version;

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
    let little_endian_hex = hex::encode(CALL_GUEST_ELF.id);
    format!("CALL_GUEST_ID: {little_endian_hex}")
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
        use regex::Regex;

        use super::*;

        #[test]
        fn guest_id_equals_to_compiled_in_version() {
            let guest_id_line = call_guest_id();
            let id_regex = Regex::new(r"^CALL_GUEST_ID: [a-f0-9]{64}$").unwrap();
            assert!(id_regex.is_match(&guest_id_line));
        }
    }
}
