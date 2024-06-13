#!/usr/bin/env bash
set -uexo pipefail

DEPLOYABLE_CONTRACT="Simple"
export STABLE_DEPLOYER_PRIV="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

ROOT_PATH=$(pwd)

DEPLOY_CONTRACTS_BASE=${ROOT_PATH}/script
DEPLOYMENT_SCRIPT=${DEPLOY_CONTRACTS_BASE}/DeployVlayer.s.sol


echo ${DEPLOYMENT_SCRIPT}

function deploy_contract() {
  results_file=$(
    forge script ${DEPLOYMENT_SCRIPT} \
      --rpc-url http://127.0.0.1:8545 \
      --broadcast 2>/dev/null \
    | grep "Transactions saved to" \
    | sed "s/^Transactions saved to: //"
  )

  jq -r ".transactions[] | select(.transactionType == \"CREATE\" and .contractName == \"${DEPLOYABLE_CONTRACT}\").contractAddress" \
    <${results_file}

}

result_address=$(deploy_contract)

echo Finished deploying contracts.
echo 
echo VLAYER_CONTRACT_ADDRESS=${result_address}