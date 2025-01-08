#!/usr/bin/env bash

set -ueo pipefail

export VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null



echo Setting up SDK 
pushd ${VLAYER_HOME}/packages/sdk
bun install --frozen-lockfile
popd

echo "::group::Building sdk"
pushd "${VLAYER_HOME}/packages/sdk"
bun run build
popd
echo '::endgroup::'

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  export EXAMPLE_NAME=$(basename "${example}")

  echo Running services...
  source ${VLAYER_HOME}/bash/run-services.sh

  echo "::group::Running tests of: ${EXAMPLE_NAME}"
  pushd "${example}"

    forge soldeer install
    forge clean
    forge build

    pushd vlayer
      bun install --frozen-lockfile
      bun run prove:"${VLAYER_ENV}"
    popd

  popd

  echo Stopping services...
  cleanup
  rm -rf "${VLAYER_TMP_DIR}/chain_db"
  echo '::endgroup::'
done

