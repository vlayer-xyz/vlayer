#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/prepare-to-lint.sh"

echo "::group::Preparing to lint"
prepare-to-lint
echo '::endgroup::'

echo "::group::Running eslint for examples"
for example in $(get_examples); do (
    echo "Running eslint for: ${example}"
    cd "$VLAYER_HOME/examples/$example/vlayer"
    bunx eslint .   
) done
echo '::endgroup::'

echo "::group::Running eslint for: $VLAYER_HOME/packages"
cd "${VLAYER_HOME}/packages"
bun run lint
echo '::endgroup::'
