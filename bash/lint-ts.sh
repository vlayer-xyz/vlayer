#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build.sh"

build_all_for_ts

echo "::group::Running eslint for examples"
for example in $(get_examples); do (
    echo "Running eslint for: ${example}"
    pushd "$VLAYER_HOME/examples/$example/vlayer"
    bunx eslint .
    popd
) done
echo '::endgroup::Running eslint for examples'

echo "::group::Running eslint for: $VLAYER_HOME/packages"
cd "${VLAYER_HOME}/packages"
bun run lint
echo '::endgroup::Running eslint for: $VLAYER_HOME/packages'
