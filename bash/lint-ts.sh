#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do

  (
    echo "Running eslint for: ${example}"
    cd "${example}/vlayer"

    bun install --frozen-lockfile
    bun run eslint .

  )
done


echo "::group::Running eslint for: $VLAYER_HOME/packages"

echo "::group::building contracts"
cd "${VLAYER_HOME}/packages/browser-extension"


echo "::group::Building sdk"
cd "${VLAYER_HOME}/packages/sdk"
bun install --frozen-lockfile
bun run build
echo '::endgroup::'
cd "${VLAYER_HOME}/packages"
bun install --frozen-lockfile
bun run lint
echo '::endgroup::'
