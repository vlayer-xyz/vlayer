#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

cd "${VLAYER_HOME}/packages/vlayer/sdk"
echo "Running tsc for: $VLAYER_HOME/packages/vlayer/sdk"
bun install
tsc  --noEmit