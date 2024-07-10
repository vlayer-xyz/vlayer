#!/usr/bin/env bash
set -ueo pipefail

RPC_URL=${RPC_URL:-http://127.0.0.1:8545}

STABLE_DEPLOYER_PRIV="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
DEPLOYER_PRIV=${DEPLOYER_PRIV:-${STABLE_DEPLOYER_PRIV}}

ROOT_PATH=$(pwd)

DEPLOY_CONTRACTS_BASE=${ROOT_PATH}/script
DEPLOYMENT_SCRIPT=${DEPLOY_CONTRACTS_BASE}/DeployVlayer.s.sol

DEPLOYABLE_CONTRACTS=("SimpleProver")

function to_upper(){
  echo $1 | sed 's/\([^A-Z]\)\([A-Z0-9]\)/\1_\2/g' | tr '[:lower:]' '[:upper:]'
}

function hex_to_decimal(){
  printf "%d\n" "$1" 
}


function deploy_contracts() {
    DEPLOYER_PRIV=${DEPLOYER_PRIV} \
    forge script ${DEPLOYMENT_SCRIPT} \
      --rpc-url ${RPC_URL} \
      --broadcast 2>/dev/null \
    | grep "Transactions saved to" \
    | sed "s/^Transactions saved to: //"
}

function retrieve_address(){
  results_file="$1"
  contract=$2

  address=$(jq -r ".transactions[] | select(.transactionType == \"CREATE\" and .contractName == \"${contract}\").contractAddress" \
    <${results_file})

  echo "$(to_upper ${contract})_ADDRESS=${address}"
  export $(to_upper ${contract})_ADDRESS=${address}
}

function get_block_number() {

  hex_number=$(
    curl -sX POST "${RPC_URL}" \
    -H 'Content-Type: application/json' \
    --data '{"id": 1, "jsonrpc": "2.0", "method": "eth_blockNumber", "params": []}' \
    | jq -r .result
  )

  hex_to_decimal "${hex_number}"

}

echo
echo "Running deployment script: ${DEPLOYMENT_SCRIPT}"

echo
echo Cleaning up old artifacts...
forge clean

echo 
echo Deploying contracts...
results_file=$(deploy_contracts)
echo Contracts have been deployed ✅
echo

for contract in "${DEPLOYABLE_CONTRACTS[@]}" ; do
  retrieve_address "${results_file}" "${contract}"
done

echo BLOCK_NO=$(get_block_number)
export BLOCK_NO=$(get_block_number)
