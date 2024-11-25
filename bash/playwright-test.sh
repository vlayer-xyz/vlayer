#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

echo Run services
source ${VLAYER_HOME}/bash/run-services.sh

echo Install dependencies
cd ${VLAYER_HOME}/packages && bun install --frozen-lockfile && cd -

echo Build SDK
cd ${VLAYER_HOME}/packages/sdk && bun run build && cd -

echo Install dependencies of test-web-app
cd ${VLAYER_HOME}/packages/test-web-app && bun install --frozen-lockfile && cd -

echo Build extension
cd ${VLAYER_HOME}/packages/browser-extension && bun run build && cd -

echo Install playwright browsers
cd ${VLAYER_HOME}/packages/browser-extension && bunx playwright install --with-deps chromium && cd -

echo Mock ImageId.sol
source ${VLAYER_HOME}/bash/mock-imageid.sh

echo Run Forge build vlayer
cd ${VLAYER_HOME}/contracts/vlayer
forge soldeer install
forge build --sizes
cd -

echo Run Forge build fixtures
cd ${VLAYER_HOME}/contracts/fixtures
forge soldeer install
forge build --sizes
cd -

echo Run playwright tests
cd ${VLAYER_HOME}/packages && bun run test:headless && cd -

