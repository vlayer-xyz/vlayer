#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

for example in $(get_examples); do
  echo "::group::Running vlayer test in: ${example}"
  cd "${VLAYER_HOME}/examples/${example}"
  forge soldeer install
  forge clean
  forge build

  if [ -d "test" ]; then
    if [ -n "${CARGO_TARGET_DIR:-}" ]; then
      "${CARGO_TARGET_DIR}/debug/vlayer" test
    else
      cargo run --manifest-path ../../Cargo.toml --package cli -- test
    fi
  else
    echo "Skipping vlayer test as test directory does not exist in ${example}"
  fi
  echo "::endgroup::"
done
