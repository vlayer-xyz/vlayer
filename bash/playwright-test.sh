#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

echo Run services
source ${VLAYER_HOME}/bash/run-services.sh

echo Mock ImageId.sol
source ${VLAYER_HOME}/bash/mock-imageid.sh

echo Run Forge build vlayer
pushd ${VLAYER_HOME}/contracts/vlayer
forge soldeer install
forge build --sizes
popd

echo Run Forge build fixtures
pushd ${VLAYER_HOME}/contracts/fixtures
forge soldeer install
forge build --sizes
popd

echo Run playwright tests
pushd ${VLAYER_HOME}/packages && bun run test:headless && popd

