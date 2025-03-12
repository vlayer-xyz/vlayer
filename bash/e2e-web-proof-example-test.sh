#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build_packages.sh"

set_proving_mode

echo '::group::Generating typescript bidings'
${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null
echo '::endgroup::'

echo '::group::Running services'
source ${VLAYER_HOME}/bash/run-services.sh
echo '::endgroup::'

build_extension

echo '::group::Running tests of: simple-web-proof'
cd "$VLAYER_HOME/examples/simple-web-proof"
forge build

cd vlayer
bun install --frozen-lockfile
bun run test-web:"${VLAYER_ENV}"
echo '::endgroup::'

echo '::group::Cleanup'
cleanup
echo '::endgroup::'
