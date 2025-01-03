use std::env;

use vergen_gitcl::{
    BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder, SysinfoBuilder,
};

pub fn emit_version_from_git_sha() {
    if env::var("VLAYER_RELEASE").is_err() {
        println!("cargo:rustc-env=VLAYER_RELEASE=dev");
    }

    emit_version_from_git_sha_inner()
        .expect("unexpected I/O error when emitting version from git sha");
}

fn emit_version_from_git_sha_inner() -> anyhow::Result<()> {
    let mut git_cl_builder = GitclBuilder::default();
    git_cl_builder.all().sha(true);

    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&git_cl_builder.build()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .emit()
}

pub fn version() -> String {
    let build_date = env::var("VERGEN_BUILD_DATE").expect(
        "VERGEN_BUILD_DATE is not defined!
Did you forget to call version::emit_version_from_git_sha() in your build.rs script?",
    );
    let build_date_without_dashes = build_date.replace('-', "");

    [
        env!("CARGO_PKG_VERSION"),
        env::var("VLAYER_RELEASE")
            .expect(
                "VLAYER_RELEASE is not defined!
Did you forget to call version::emit_version_from_git_sha() in your build.rs script?",
            )
            .as_str(),
        build_date_without_dashes.as_str(),
        env::var("VERGEN_GIT_SHA")
            .expect(
                "VERGEN_GIT_SHA is not defined!
Did you forget to call version::emit_version_from_git_sha() in your build.rs script?",
            )
            .as_str(),
    ]
    .join("-")
}
