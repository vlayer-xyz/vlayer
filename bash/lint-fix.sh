#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

for example in $(get_examples); do
  (
    cd "$VLAYER_HOME/examples/$example/vlayer"

    bun install --frozen-lockfile --no-save --silent
    bun run --silent eslint . --fix
    bun run --silent lint:solidity -- --fix --noPrompt
  )
done

cd "${VLAYER_HOME}/packages"
bun install --frozen-lockfile --no-save --silent
bun run --silent lint:fix

cd "${VLAYER_HOME}/contracts/vlayer"
bun install --frozen-lockfile --no-save --silent
bun run --silent lint:solidity -- --fix --noPrompt

forge fmt "${VLAYER_HOME}/examples" -- --write
forge fmt "${VLAYER_HOME}/contracts/vlayer/src" -- --write
