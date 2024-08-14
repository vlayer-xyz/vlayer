use clap::builder::{IntoResettable, Resettable, Str};

use call_guest_wrapper::RISC0_CALL_GUEST_ID;

pub struct Version;

impl IntoResettable<Str> for Version {
    fn into_resettable(self) -> Resettable<Str> {
        version_msg()
    }
}

fn version_msg() -> Resettable<Str> {
    [build_msg(), call_guest_id_msg()]
        .join("\n")
        .into_resettable()
}

fn call_guest_id_msg() -> String {
    let call_guest_id_hex = RISC0_CALL_GUEST_ID.map(|e| format!("{e:x}")).join("");
    format!("CALL_GUEST_ID: {}", call_guest_id_hex)
}

fn build_msg() -> String {
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
