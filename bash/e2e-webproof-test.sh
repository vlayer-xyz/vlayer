#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

cd "${VLAYER_HOME}/packages/"
echo "Running e2e test for browser extension"
bun install
cd "${VLAYER_HOME}/packages/browser-extension/"
echo 'going to install playwright deps'
bunx playwright install --with-deps
bun run test:headless

