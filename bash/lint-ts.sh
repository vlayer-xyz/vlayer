#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)


for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do

  (
    echo "Running eslint for: ${example}"
    cd "${example}/vlayer"

    bun install
    bun run eslint .

  )
done

echo "Running eslint for: $VLAYER_HOME/packages"
cd "${VLAYER_HOME}/packages"

bun install
bun run lint

cd "${VLAYER_HOME}/packages/vlayer/sdk"
echo "Running tsc  for: $VLAYER_HOME/packages/vlayer/sdk"
tsc  --noEmit