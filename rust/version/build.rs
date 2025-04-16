use std::env;

use vergen_gitcl::{
    BuildBuilder, CargoBuilder, Emitter, GitclBuilder, RustcBuilder, SysinfoBuilder,
};

#[allow(clippy::expect_used)]
fn main() {
    println!("cargo:rerun-if-env-changed=VLAYER_RELEASE");

    if env::var("VLAYER_RELEASE").is_err() {
        println!("cargo:rustc-env=VLAYER_RELEASE=dev");
    }

    emit_version_from_git_sha().expect("unexpected I/O error when emitting version from git sha");
}

fn emit_version_from_git_sha() -> anyhow::Result<()> {
    let mut git_cl_builder = GitclBuilder::default();
    git_cl_builder.all().sha(false);

    Emitter::default()
        .add_instructions(&BuildBuilder::all_build()?)?
        .add_instructions(&CargoBuilder::all_cargo()?)?
        .add_instructions(&git_cl_builder.build()?)?
        .add_instructions(&RustcBuilder::all_rustc()?)?
        .add_instructions(&SysinfoBuilder::all_sysinfo()?)?
        .emit()
}
