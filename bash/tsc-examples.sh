#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

cd "${VLAYER_HOME}/packages"
echo "Running bun install for: $VLAYER_HOME/packages"
bun install --frozen-lockfile

EXAMPLE="${VLAYER_HOME}/examples/web_proof"
cd ${EXAMPLE}

forge soldeer install
forge build

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build_ts_types.sh >/dev/null

cd ${EXAMPLE}/vlayer

echo "Running tsc for: ${EXAMPLE}"
bun install --frozen-lockfile
bun tsc --noEmit
