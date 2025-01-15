#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

echo "::group::Installing npm dependencies"
cd "${VLAYER_HOME}"
bun install --frozen-lockfile
echo '::endgroup::'

echo "::group::Building sdk"
cd "${VLAYER_HOME}/packages/sdk"
bun run build
echo '::endgroup::'

for example in $(get_examples); do
  echo ""::group::Running tsc for: ${example}""
  example_path="${VLAYER_HOME}/examples/${example}"
  
  cd $example_path

  forge soldeer install
  forge build

  echo Generating typescript bidings ...
  ${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null

  cd "${example_path}/vlayer"

  bun install --frozen-lockfile
  bun tsc --noEmit
  echo '::endgroup::'
done
