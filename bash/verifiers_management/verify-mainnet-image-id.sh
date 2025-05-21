#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
NETWORKS=( "ethereum" "optimism" "base" )

set -ueo pipefail

SCRIPT="ImageIdAdministration.s.sol"
CONTRACT="VerifyImageIdSupported"
SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${SCRIPT}:${CONTRACT}"

cd "${CONTRACTS_DIR}"

cleanup

for NETWORK in "${NETWORKS[@]}" ; do
  run_forge_script "${SCRIPT_INVOCATION}" "${NETWORK}"
done
