#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/common.sh"

set -ueo pipefail

DEPLOYER_SCRIPT="VlayerDeployer.s.sol"
DEPLOYER_CONTARCT="VlayerDeployer"
DEPLOYER_SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${DEPLOYER_SCRIPT}:${DEPLOYER_CONTARCT}"

function deploy_contracts(){
  run_forge_script "${DEPLOYER_SCRIPT_INVOCATION}" 
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
