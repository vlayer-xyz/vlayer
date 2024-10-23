#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do

  (
    echo "Running solhint of: ${example}"
    cd "${example}/vlayer"

    bun install --frozen-lockfile
    bun run lint:solidity
  )
done
 
