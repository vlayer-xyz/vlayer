#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build_ts_types.sh >/dev/null

echo Running services...
source ${VLAYER_HOME}/bash/run-services.sh

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do

  (
    echo "Running tests of: ${example}"
    cd "${example}"
    echo "Running example: ${example}"
    forge soldeer install
    cd vlayer

    forge clean
    forge build

    bun install
    bun run prove.ts 
  )
done
 
