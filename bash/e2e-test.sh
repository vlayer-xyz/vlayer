#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build_ts_types.sh >/dev/null

echo Running services...
source ${VLAYER_HOME}/bash/run-services.sh

echo Setting up SDK 
cd ${VLAYER_HOME}/packages/sdk && bun install

EXAMPLES_REQUIRING_ALCHEMY=("simple_time_travel" "simple_teleport")
EXAMPLES_REQUIRING_PRIV_KEY=("simple_time_travel")

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  example_name=$(basename "${example}")

  if [[ "${EXAMPLES_REQUIRING_ALCHEMY[@]}" =~ "${example_name}" ]] && [[ -z "${ALCHEMY_API_KEY:-}" ]]; then
    echo "Skipping: ${example} (configure ALCHEMY_API_KEY to run it)"
    continue
  fi

  if [[ "${EXAMPLES_REQUIRING_PRIV_KEY[@]}" =~ "${example_name}" ]] && [[ -z "${EXAMPLES_TEST_PRIVATE_KEY:-}" ]]; then
    echo "Skipping: ${example} (configure EXAMPLES_TEST_PRIVATE_KEY to run it)"
    continue
  fi

  echo "Running tests of: ${example}"
  cd "${example}"
  forge soldeer install
  forge clean
  forge build

  cd vlayer
  bun install
  bun run prove.ts 
done

