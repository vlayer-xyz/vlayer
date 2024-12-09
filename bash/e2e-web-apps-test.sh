#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null

echo Running services...
source ${VLAYER_HOME}/bash/run-services.sh

echo Setting up SDK 
cd ${VLAYER_HOME}/packages/sdk && bun install --frozen-lockfile

echo "::group::Building sdk"
cd "${VLAYER_HOME}/packages/sdk"
bun run build
echo '::endgroup::'

echo 'going to install playwright deps'
bunx playwright install chromium --with-deps

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  example_name=$(basename "${example}")

  echo "::group::Running tests of: ${example}"
  cd "${example}"
  forge soldeer install
  forge clean
  forge build

  cd vlayer
  bun install --frozen-lockfile
  bun run test:"${VLAYER_ENV}"
  echo '::endgroup::'
done

