#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

bun install --frozen-lockfile

for example in $(get_examples); do (
    echo "Running solhint of: ${example}"
    cd "$VLAYER_HOME/examples/$example/vlayer"
    bun run lint:solidity --max-warnings 0
) done
 
