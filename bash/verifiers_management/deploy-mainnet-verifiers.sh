#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
NETWORKS=( "ethereum" "optimism" "base")

set -ueo pipefail

DEPLOYER_SCRIPT="MainnetVlayerDeployer.s.sol"
DEPLOYER_CONTRACT="MainnetVlayerDeployer"
DEPLOYER_SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${DEPLOYER_SCRIPT}:${DEPLOYER_CONTRACT}"

function deploy_contracts(){
  for NETWORK in "${NETWORKS[@]}" ; do
    run_forge_script "${DEPLOYER_SCRIPT_INVOCATION}" "${NETWORK}"
  done
}

cd "${CONTRACTS_DIR}"

cleanup
build_contracts
deploy_contracts
