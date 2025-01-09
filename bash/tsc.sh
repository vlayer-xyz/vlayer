#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

bun install --frozen-lockfile
cd "${VLAYER_HOME}/packages"
echo "Running tsc for: $VLAYER_HOME/packages"
bun tsc --build --noEmit
