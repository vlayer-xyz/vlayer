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

SDK_DIR="$VLAYER_HOME/packages/vlayer/sdk"
echo "Running eslint for: $SDK_DIR"
cd "${SDK_DIR}"

bun install
bun run eslint .