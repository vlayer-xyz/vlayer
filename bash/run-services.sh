#!/usr/bin/env bash

set -ueo pipefail

if [[ -z "${VLAYER_TMP_DIR:-}" ]] ; then
    VLAYER_TMP_DIR=$(mktemp -d -t vlayer-$(basename $0).XXXXX)
else
    VLAYER_TMP_DIR=$(realpath "${VLAYER_TMP_DIR}")
fi

LOGS_DIR="${VLAYER_TMP_DIR}/logs"
VLAYER_HOME=$(git rev-parse --show-toplevel)

source "${VLAYER_HOME}/bash/common.sh"

mkdir -p "${LOGS_DIR}"
echo "Saving artifacts to: ${VLAYER_TMP_DIR}"

CHAIN_WORKERS=()

function cleanup() {
    echo "Cleaning up..."

    for service in ANVIL CHAIN_SERVER VLAYER_SERVER ; do 
        kill_service "${service}"
    done

    for worker in "${CHAIN_WORKERS[@]}" ; do
        kill_service "${worker}"
    done  
}

function start_anvil(){
    echo "Starting Anvil"
    startup_anvil "${LOGS_DIR}/anvil.out" 8545 ANVIL
}

function startup_vlayer(){
    local proof_arg=$1
    shift # shift input params, since the second (and last) arg is an array of external_urls 
    local external_urls=("$@")

    echo "Starting vlayer REST server"
    pushd "${VLAYER_HOME}/rust"

    RUST_LOG=info \
    BONSAI_API_URL="${BONSAI_API_URL}" \
    BONSAI_API_KEY="${BONSAI_API_KEY}" \
    cargo run --bin vlayer serve \
        --proof "${proof_arg}" \
        --chain-proof-url http://localhost:3001 \
        --rpc-url 31337:http://localhost:8545 \
        ${external_urls[@]+"${external_urls[@]}"} \
        >"${LOGS_DIR}/vlayer_serve.out" &

    VLAYER_SERVER=$!

    echo "vlayer server started with PID ${VLAYER_SERVER}."
    wait_for_port_and_pid 3000 ${VLAYER_SERVER} 30m "vlayer server"

    popd
}

function startup_chain_services() {
    local db_path="${VLAYER_TMP_DIR}/chain_db"

    for args in "$@"; do 
        startup_chain_worker ${db_path} $args &
    done

    startup_chain_server ${db_path}

    for args in "$@"; do 
        wait_for_chain_worker_sync $args &
    done
}

function startup_chain_worker() {
    local db_path="$1"
    local rpc_url="$2"
    local chain_id="$3"
    local first_block="$4"
    local last_block="$5"

    echo "Starting chain worker with rpc_url=${rpc_url} chain_id=${chain_id}"
    pushd "${VLAYER_HOME}/rust"
    
    local start_block=$(( ($first_block + $last_block) / 2 ))

    RUST_LOG=${RUST_LOG:-info} \
    cargo run --bin worker -- \
        --db-path "${db_path}" \
        --rpc-url "${rpc_url}" \
        --chain-id "${chain_id}" \
        --confirmations "${CONFIRMATIONS:-1}" \
        --start-block "${start_block}" \
        --max-head-blocks "${MAX_HEAD_BLOCKS:-10}" \
        --max-back-propagation-blocks "${MAX_BACK_PROPAGATION_BLOCKS:-10}" \
        >"${LOGS_DIR}/chain_worker_${chain_id}.out" 2>&1 &

    local worker_pid=$!
    echo "Chain worker started with PID ${worker_pid}."
    CHAIN_WORKERS+=($worker_pid)

    popd
}

function startup_chain_server() {
    local db_path="$1"

    echo "Starting chain server"
    pushd "${VLAYER_HOME}/rust"

    RUST_LOG=info \
    cargo run --bin chain_server -- \
        --db-path "${db_path}" \
        >"${LOGS_DIR}/chain_server.out" &

    CHAIN_SERVER=$!
    
    echo "Chain server started with PID ${CHAIN_SERVER}."
    wait_for_port_and_pid 3001 "${CHAIN_SERVER}" 30m "chain server"

    popd
}


wait_for_chain_worker_sync() {
    local _rpc_url="$1"  # Ignored, only present for compatibility with CHAIN_WORKER_ARGS
    local chain_id="$2"
    local first_block="$3"
    local last_block="$4"
    echo "Waiting for chain worker sync... chain_id=${chain_id} first_block=${first_block} last_block=${last_block}"

    for i in `seq 1 10` ; do

        reply=$(curl -s -X POST 127.0.0.1:3001 \
                  --retry-connrefused \
                  --retry 5 \
                  --retry-delay 0 \
                  --retry-max-time 30 \
                  -H "Content-Type: application/json" \
                  --data '{"jsonrpc": "2.0","id": 0,"method": "v_sync_status","params": ['"${chain_id}"']}'
        )
        synced=$(echo "${reply}" | jq "(.result.first_block <= ${first_block}) and (.result.last_block >= ${last_block})")

        if [[ "${synced}" == "true" ]] ; then
            echo "Chain ${chain_id} worker synced"
            break
        fi
        echo "Syncing ... ${reply}"
        sleep 10
    done

    if [[ "${synced}" != "true" ]] ; then
        echo "Failed to sync chain ${chain_id} worker" >&2
        exit 1 
    fi

}


trap cleanup EXIT ERR INT

# Default values
PROVING_MODE=${PROVING_MODE:-dev}
RISC0_DEV_MODE=""
BONSAI_API_URL="${BONSAI_API_URL:-https://api.bonsai.xyz/}"
BONSAI_API_KEY="${BONSAI_API_KEY:-}"
SERVER_PROOF_ARG="fake"
EXTERNAL_RPC_URLS=()

# Set the SERVER_PROOF_MODE variable based on the mode
if [[ "${PROVING_MODE}" == "dev" ]]; then
    SERVER_PROOF_ARG="fake"
elif [[ "${PROVING_MODE}" == "prod" ]]; then
    SERVER_PROOF_ARG="groth16"

    # Check that BONSAI_API_URL and BONSAI_API_KEY are not empty in prod mode
    if [[ -z "$BONSAI_API_URL" || -z "$BONSAI_API_KEY" ]]; then
        echo "Error: BONSAI_API_URL and BONSAI_API_KEY must be set in prod mode."
        usage
    fi
fi

# set external rpc urls
if [[ -z "${ALCHEMY_API_KEY:-}" ]] ; then
    echo ALCHEMY_API_KEY is not configured. Using only local rpc-urls. >&2
    CHAIN_WORKER_ARGS=(
        "http://localhost:8545 31337 18000000 18000500"
    )
else
    CHAIN_WORKER_ARGS=(
        "http://localhost:8545 31337 latest latest"
        "https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY} 1 20683110 20683110"
        "https://base-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY} 8453 19367633 19367633"
        "https://opt-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY} 10 124962954 124962954"
    )

    EXTERNAL_RPC_URLS=(
        "--rpc-url" "11155111:https://eth-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "1:https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}" 
        "--rpc-url" "8453:https://base-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "10:https://opt-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "11155420:https://opt-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "84532:https://base-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "80002:https://polygon-amoy.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "421614:https://arb-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "300:https://zksync-sepolia.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "8453:https://base-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "42161:https://arb-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "42170:https://arbnova-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "137:https://polygon-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "324:https://zksync-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
    )

fi

# Display the parsed parameters
echo "PROVING_MODE: ${PROVING_MODE}"
echo "BONSAI_API_URL: ${BONSAI_API_URL}"
echo "SERVER_PROOF_ARG: ${SERVER_PROOF_ARG}"
echo "EXTERNAL_RPC_URLS: ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}"
echo
echo "Starting services..."

start_anvil
if [[ "${RUN_CHAIN_SERVICES:-0}" == "1" ]] ; then 
    startup_chain_services "${CHAIN_WORKER_ARGS[@]}"
fi
startup_vlayer "${SERVER_PROOF_ARG}" ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}

echo "Services have been successfully started"
