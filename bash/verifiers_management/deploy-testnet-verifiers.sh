#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
NETWORKS=( "base" "sepolia" "base-sepolia" "optimism-sepolia" "arbitrum-sepolia" "worldchain-sepolia" )

set -ueo pipefail

DEPLOYER_SCRIPT="TestnetVlayerDeployer.s.sol"
DEPLOYER_CONTRACT="TestnetVlayerDeployer"
DEPLOYER_SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${DEPLOYER_SCRIPT}:${DEPLOYER_CONTRACT}"

function deploy_contracts(){
  for NETWORK in "${NETWORKS[@]}" ; do
    run_forge_script "${DEPLOYER_SCRIPT_INVOCATION}" "${NETWORK}"
  done
}

function verify_deployment_is_stable_across_networks(){
  local stable_deployment=$(cat deployed_contracts.json | jq -r)
  for network_txs in $(find "${CONTRACTS_DIR}/broadcast/${DEPLOYER_SCRIPT}" -type f -name "run-latest.json") ; do
    result=$(jq <"${network_txs}" -r '{contracts: .transactions | map({contractName: .contractName, contractAddress: .contractAddress})}')
    if [[ "${stable_deployment}" != "${result}" ]] ; then
      echo "Deployments are not consistent ❌" >&2
      echo "--------------------------" >&2
      echo "${stable_deployment}" | jq  >&2
      echo "--------------------------" >&2
      echo "${result}" | jq >&2
      echo "--------------------------" >&2
      exit 1
    fi
  done
  echo "Deployments has been verified ✅"
}

cd "${CONTRACTS_DIR}"

cleanup
build_contracts
deploy_contracts
verify_deployment_is_stable_across_networks
