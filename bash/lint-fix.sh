#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  (
    cd "${example}/vlayer"

    bun install --no-save --silent
    bun run --silent eslint . --fix
    bun run --silent lint:solidity -- --fix --noPrompt
  )
done

cd "${VLAYER_HOME}/packages"
bun install --no-save --silent
bun run --silent lint:fix

cd "${VLAYER_HOME}/contracts"
bun install --no-save --silent
bun run --silent lint:solidity -- --fix --noPrompt

forge fmt "${VLAYER_HOME}/examples" -- --write
forge fmt "${VLAYER_HOME}/contracts/src" -- --write
