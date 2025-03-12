#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-contracts.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/utils.sh"

echo "::group::Installing npm dependencies"
cd "${VLAYER_HOME}"
bun install --frozen-lockfile
echo "::endgroup::Installing npm dependencies"

build_sdk
build_sdk_hooks


for example in $(get_examples); do
  echo "::group::Running tsc for: ${example}"

  build_example_contracts $example

  generate_ts_bindings

  pushd "${VLAYER_HOME}/examples/${example}/vlayer"
  bun install --frozen-lockfile
  bun tsc --noEmit
  popd

  echo "::endgroup::Running tsc for: ${example}"
done
