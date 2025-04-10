#!/usr/bin/env bash
set -euo pipefail
VLAYER_HOME=$(git rev-parse --show-toplevel)

static_check_workspace() {
    cargo +nightly fmt --all --check
    cargo sort --check --grouped --workspace
    cargo clippy --all-targets --all-features --locked -- -D warnings
}

(cd rust/cli && cargo deny check bans)
cargo machete

(cd $VLAYER_HOME && static_check_workspace)
(cd $VLAYER_HOME/rust/guest_wrapper/risc0_chain_guest && static_check_workspace)
(cd $VLAYER_HOME/rust/guest_wrapper/risc0_call_guest && static_check_workspace)
