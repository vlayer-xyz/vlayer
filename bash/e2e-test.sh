#!/usr/bin/env bash

set -ueo pipefail

# Imports
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib"

# Defaults
PROVING_MODE=${PROVING_MODE:-dev}
VLAYER_ENV=${VLAYER_ENV:-dev}

generate_ts_bindings
build_sdk

echo "::group::Running examples"
for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
  export EXAMPLE_NAME=$(basename "${example}")
    
  echo Running services...
  source ${VLAYER_HOME}/bash/run-services.sh

  pushd "${example}"
  echo "::group::Running tests of: ${example}"
  silent_unless_fails build_contracts
  run_prover_script
  popd

  cleanup
  rm -rf "${VLAYER_TMP_DIR}/chain_db"
done
echo '::endgroup::Running examples'