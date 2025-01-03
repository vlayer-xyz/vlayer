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

#[macro_export]
macro_rules! version {
    () => {{
        let build_date = env!("VERGEN_BUILD_DATE").replace("-", "");
        [
            env!("CARGO_PKG_VERSION"),
            env!("VLAYER_RELEASE"),
            build_date.as_str(),
            env!("VERGEN_GIT_SHA"),
        ]
        .join("-")
    }};
}
