#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

bun install --frozen-lockfile

for example in $(get_examples); do (
    echo "Running eslint for: ${example}"
    cd "$VLAYER_HOME/examples/$example/vlayer"
    bunx eslint .   
) done


echo "::group::Running eslint for: $VLAYER_HOME/packages"

echo "::group::building contracts"
cd "${VLAYER_HOME}/packages/browser-extension"
echo '::endgroup::'

echo "::group::Building sdk"
cd "${VLAYER_HOME}/packages/sdk"
bun install --frozen-lockfile
bun run build
echo '::endgroup::'

cd "${VLAYER_HOME}/packages"
bun install --frozen-lockfile
bun run lint
echo '::endgroup::'
