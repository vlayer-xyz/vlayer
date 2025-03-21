#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

source "$VLAYER_HOME/bash/lib/examples.sh"

for example in $(get_examples); do (
    pushd "$VLAYER_HOME/examples/$example/vlayer"
    if grep -q "test:unit" package.json; then
        echo "Running unit tests for: ${example}"
        bun run test:unit
    fi
    popd
) done
