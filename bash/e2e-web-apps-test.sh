#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null

for example in $(get_examples); do

  echo Running services...
  source ${VLAYER_HOME}/bash/run-services.sh

  echo "::group::Running tests of: ${example}"
  cd "$VLAYER_HOME/examples/$example"
  forge build

  cd vlayer
  bun install --frozen-lockfile
  bun run test:"${VLAYER_ENV}"

  cleanup
done

