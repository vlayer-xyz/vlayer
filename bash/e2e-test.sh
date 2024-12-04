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

EXAMPLES_REQUIRING_ALCHEMY=("simple_teleport")
# Only run limited selection of examples in prod mode,
# because they use real Bonsai resources.
EXAMPLES_RUN_IN_PROD_MODE=("simple")

echo "::group::Building sdk"
cd "${VLAYER_HOME}/packages/sdk"
bun run build
echo '::endgroup::'

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  example_name=$(basename "${example}")

  if [[ "${PROVING_MODE}" = "prod" ]] && [[ ! "${EXAMPLES_RUN_IN_PROD_MODE[@]}" =~ "${example_name}" ]]; then
    echo "Skipping: ${example} - not running it in prod mode."
    continue
  fi

  if [[ "${EXAMPLES_REQUIRING_ALCHEMY[@]}" =~ "${example_name}" ]] && [[ -z "${ALCHEMY_API_KEY:-}" ]]; then
    echo "Skipping: ${example} (configure ALCHEMY_API_KEY to run it)"
    continue
  fi

  echo "::group::Running tests of: ${example}"
  cd "${example}"
  forge soldeer install
  forge clean
  forge build

  cd vlayer
  bun install --frozen-lockfile
  bun run prove:"${VLAYER_ENV}"
  echo '::endgroup::'
done

