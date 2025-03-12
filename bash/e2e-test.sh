#!/usr/bin/env bash

set -ueo pipefail

# Imports
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"

# Defaults
set_proving_mode
VLAYER_ENV=${VLAYER_ENV:-dev}

generate_ts_bindings
build-sdk

BUN_NO_FROZEN_LOCKFILE=1
export EXAMPLE_NAME=$EXAMPLE

echo "::group::Running services"
source ${VLAYER_HOME}/bash/run-services.sh
echo "::endgroup::Running services"

cd $(mktemp -d)

generate_vlayer_init_config
init_template

pushd $EXAMPLE

silent_unless_fails forge build
run_prover_script

popd

cleanup
rm -rf "${VLAYER_TMP_DIR}/chain_db"
