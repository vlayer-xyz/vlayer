#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  echo "::group::Running vlayer test in: ${example}"
  cd "${example}"
  forge soldeer install
  forge clean
  forge build

  if [ -d "test" ]; then
    cargo run --manifest-path ../../rust/Cargo.toml --package cli -- test
  else
    echo "Skipping vlayer test as test directory does not exist in ${example}"
  fi
  echo '::endgroup::'
done
