#!/usr/bin/env bash
set -euo pipefail

cargo +nightly fmt --all --check
cargo sort --check --grouped --workspace
cargo clippy --all-targets --all-features --locked -- -D warnings
