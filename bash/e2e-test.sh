#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build_ts_types.sh >/dev/null

echo Running services...
source ${VLAYER_HOME}/bash/run-services.sh

echo Setting up SDK 
cd ${VLAYER_HOME}/packages/vlayer/sdk && bun install

# these examples require ALCHEMY_API_KEY, which may not be available locally, so we don't run them
# if the key is not available
EXAMPLES_REQUIRING_ALCHEMY=("simple_time_travel" "simple_teleport")

# check if ALCHEMY_API_KEY is set in GitHub actions; running in GH is detected by checking RUNNER_OS env var
if [[  -n "${RUNNER_OS:-}" ]] && [[ -z "${ALCHEMY_API_KEY:-}" ]] ;then 
  echo "ALCHEMY_API_KEY must be set in GitHub actions. Exiting." >&2
  exit 1
fi

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do

  (
    example_name=$(basename "${example}")
    if [[ "${EXAMPLES_REQUIRING_ALCHEMY[@]}" =~ "${example_name}" ]] && [[ -z "${ALCHEMY_API_KEY:-}" ]]; then
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

  )
done

