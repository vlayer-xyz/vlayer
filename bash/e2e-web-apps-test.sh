#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

PROVING_MODE=${PROVING_MODE:-dev}

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null

echo Running services...
source ${VLAYER_HOME}/bash/run-services.sh

for example in $(get_examples); do
  echo "make snapshot of anvil"
  ANVIL_SNAPSHOT_ID=$(cast rpc evm_snapshot)

  echo "::group::Running tests of: ${example}"
  cd "$VLAYER_HOME/examples/$example"
  forge soldeer install
  forge clean
  forge build

  cd vlayer
  bun install --frozen-lockfile
  bun run test:"${VLAYER_ENV}"

  echo "revert anvil to initial state"
  cast rpc evm_revert "${ANVIL_SNAPSHOT_ID}"
  echo '::endgroup::'
done

