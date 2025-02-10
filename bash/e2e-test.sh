#!/usr/bin/env bash

set -ueo pipefail

# Imports
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib.sh"

# Defaults
set_proving_mode
VLAYER_ENV=${VLAYER_ENV:-dev}

generate_ts_bindings
build_sdk
build_vlayer_contracts

echo "::group::Running examples"
for example in $(get_examples); do
  export EXAMPLE_NAME=$example
  echo Running services...
  source ${VLAYER_HOME}/bash/run-services.sh

  pushd "$VLAYER_HOME/examples/$example"
  echo "::group::Running tests of: ${example}"
  silent_unless_fails build_contracts
  run_prover_script
  popd

  cleanup
  rm -rf "${VLAYER_TMP_DIR}/chain_db"
done
echo '::endgroup::Running examples'