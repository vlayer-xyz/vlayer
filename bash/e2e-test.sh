#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

source ${VLAYER_HOME}/bash/run-services.sh

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do

  (
    echo "Running tests of: ${example}"
    cd "${example}/vlayer"

    forge clean
    forge build

    bun install
    bun run prove.ts 

  )
done
 
