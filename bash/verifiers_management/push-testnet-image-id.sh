#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
NETWORKS=( "sepolia" "base-sepolia" "optimism-sepolia" "arbitrum-sepolia" "worldchain-sepolia" "base" "flow-evm-testnet" )

# Simple function to get Repository address from .sol file
repository_address() {
    grep -o "Repository(address(0x[a-fA-F0-9]\{40\}))" "${CONTRACTS_DIR}/src/TestnetStableDeployment.sol" | \
    grep -o "0x[a-fA-F0-9]\{40\}"
}

set -ueo pipefail

SCRIPT="ImageIdAdministration.s.sol"
CONTARCT="AddSupportForCurrentImageId"
SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${SCRIPT}:${CONTARCT}"
REPOSITORY_ADDRESS=$(repository_address)

cd "${CONTRACTS_DIR}"

cleanup

for NETWORK in "${NETWORKS[@]}" ; do
  REPOSITORY_CONTRACT_ADDRESS=${REPOSITORY_ADDRESS} run_forge_script "${SCRIPT_INVOCATION}" "${NETWORK}"
done
