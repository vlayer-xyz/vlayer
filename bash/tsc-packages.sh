#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

echo "::group::Installing npm dependencies"
pushd "${VLAYER_HOME}"
bun install --frozen-lockfile
popd
echo "::endgroup::Installing npm dependencies"

echo "::group::Running tsc for: ${VLAYER_HOME}/packages"
pushd "${VLAYER_HOME}/packages"
bun tsc --build --noEmit
popd
echo "::endgroup::Running tsc for: ${VLAYER_HOME}/packages"
