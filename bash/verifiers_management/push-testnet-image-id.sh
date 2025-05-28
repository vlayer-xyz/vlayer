#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
NETWORKS=( "sepolia" "base-sepolia" "optimism-sepolia" "arbitrum-sepolia" "worldchain-sepolia" "base" )

set -ueo pipefail

SCRIPT="ImageIdAdministration.s.sol"
CONTARCT="AddSupportForCurrentImageId"
SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${SCRIPT}:${CONTARCT}"
REPOSITORY_ADDRESS=$(jq -r <"${CONTRACTS_DIR}/deployed_contracts.json" '.contracts[] | select(.contractName == "Repository") | .contractAddress')

cd "${CONTRACTS_DIR}"

cleanup

for NETWORK in "${NETWORKS[@]}" ; do
  REPOSITORY_CONTRACT_ADDRESS=${REPOSITORY_ADDRESS} run_forge_script "${SCRIPT_INVOCATION}" "${NETWORK}"
done
