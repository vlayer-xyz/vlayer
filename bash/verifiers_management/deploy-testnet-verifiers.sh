#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
NETWORKS=( "sepolia" "base-sepolia" "optimism-sepolia" "arbitrum-sepolia" "worldchain-sepolia" )

set -ueo pipefail

DEPLOYER_SCRIPT="TestnetVlayerDeployer.s.sol"
DEPLOYER_CONTRACT="TestnetVlayerDeployer"
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
