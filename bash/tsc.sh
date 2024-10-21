#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

cd "${VLAYER_HOME}/packages"
echo "Running tsc for: $VLAYER_HOME/packages"
bun install --frozen-lockfile
bun tsc --build --noEmit
