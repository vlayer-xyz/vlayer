#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
CONTRACTS_DIR="${VLAYER_HOME}/contracts/vlayer"
DEPLOYER_SCRIPT="VerifierDeployer.s.sol"
DEPLOYER_CONTARCT="VerifierDeployer"
DEPLOYER_SCRIPT_INVOCATION="${CONTRACTS_DIR}/script/${DEPLOYER_SCRIPT}:${DEPLOYER_CONTARCT}"

DRY_RUN=${DRY_RUN:-true}

NETWORKS=( "sepolia" "base-sepolia" "optimism-sepolia" )
echo "DRY_RUN: ${DRY_RUN}"
echo "NETWORKS: ${NETWORKS[@]}"

function deploy_contracts(){
  local dry_run=$1
  local broadcast_flag=""
  if [[ "${dry_run}" == "false" ]]; then
      broadcast_flag="--broadcast"
  fi

  for NETWORK in "${NETWORKS[@]}" ; do
    echo "Deploying to ${NETWORK}"
    forge script "${DEPLOYER_SCRIPT_INVOCATION}" \
      --chain "${NETWORK}" \
      --rpc-url "${NETWORK}" \
      --slow \
      "${broadcast_flag}" 
  done
}

function verify_deployment_is_stable_across_networks(){
  local stable_deployment=$(cat deployed_contracts.json | jq -r)
  for network_txs in $(find "${CONTRACTS_DIR}/broadcast/${DEPLOYER_SCRIPT}" -type f -name "run-latest.json") ; do
    result=$(jq <"${network_txs}" -r '{contracts: .transactions | map({contractName: .contractName, contractAddress: .contractAddress})}')
    if [[ "${stable_deployment}" != "${result}" ]] ; then
      echo "Deployments are not consistent ❌" >&2
      echo "--------------------------" >&2
      echo "${stable_deployment}" | jq >&2
      echo "--------------------------" >&2
      echo "${result}" | jq >&2
      echo "--------------------------" >&2
      exit 1
    fi
  done
  echo "Deployments has been verified ✅"
}

function cleanup(){
  echo "Cleaning up artifacts..."
  if [[ -d "./broadcast" ]] ; then 
    rm -r ./broadcast ./cache
  fi
  forge clean

  echo "Building contracts..."
  forge build --extra-output-files metadata
}

cd "${CONTRACTS_DIR}"

cleanup
deploy_contracts "${DRY_RUN}"
verify_deployment_is_stable_across_networks
