#!/usr/bin/env bash

set -ueo pipefail

if [[ -z "${VLAYER_TMP_DIR:-}" ]] ; then
    VLAYER_TMP_DIR=$(mktemp -d -t vlayer-$(basename $0).XXXXX)
else
    VLAYER_TMP_DIR=$(realpath "${VLAYER_TMP_DIR}")
fi

LOGS_DIR="${VLAYER_TMP_DIR}/logs"
VLAYER_HOME=$(git rev-parse --show-toplevel)
BUILD_BINARIES=${BUILD_BINARIES:-1}

source "${VLAYER_HOME}/bash/common.sh"

mkdir -p "${LOGS_DIR}"
echo "Saving artifacts to: ${VLAYER_TMP_DIR}"

CHAIN_WORKER_PIDS="${VLAYER_TMP_DIR}/chain_worker_pids"
touch "${CHAIN_WORKER_PIDS}"

function cleanup() {
    echo "Cleaning up..."

    for service in ANVIL CHAIN_SERVER VLAYER_SERVER ; do 
        kill_service "${service}"
    done

    while read worker_pid; do
        if ps -p "${worker_pid}" >/dev/null; then
            echo "Killing worker ${worker_pid}"
            kill "${worker_pid}"
        fi
    done < "${CHAIN_WORKER_PIDS}"

    echo "Cleanup done. Artifacts saved to: ${VLAYER_TMP_DIR}"
}

function start_anvil(){
    echo "Starting anvil"
    startup_anvil "${LOGS_DIR}/anvil.out" 8545 ANVIL
}

function startup_vlayer(){
    local proof_arg=$1
    shift # shift input params, since the second (and last) arg is an array of external_urls 
    local external_urls=("$@")

    echo "Starting vlayer REST server"
    pushd "${VLAYER_HOME}/rust"

    local args=(
        "--proof" "${proof_arg}"
        "--rpc-url" "31337:http://localhost:8545"
    ) 
    if [[ "${RUN_CHAIN_SERVICES:-0}" == "1" ]] ; then 
        args+=("--chain-proof-url" "http://localhost:3001")
    fi

    RUST_LOG=info \
    RISC0_DEV_MODE="${RISC0_DEV_MODE}" \
    BONSAI_API_URL="${BONSAI_API_URL}" \
    BONSAI_API_KEY="${BONSAI_API_KEY}" \
    ./target/debug/vlayer serve \
        ${args[@]} \
        ${external_urls[@]+"${external_urls[@]}"} \
        >>"${LOGS_DIR}/vlayer_serve.out" &

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
        wait_for_chain_worker_sync $args
    done
}

function get_latest_block() {
    local rpc_url=$1
    local block_hex=$(curl -s ${rpc_url} \
        -X POST \
        -H "Content-Type: application/json" \
        --data '{"method":"eth_blockNumber","params":[],"id":1,"jsonrpc":"2.0"}' \
        | jq -r ".result"
    )
    printf "%d\n" "${block_hex}"
}

function startup_chain_worker() {
    local db_path="$1"
    local rpc_url="$2"
    local chain_id="$3"
    local first_block="$4"
    local last_block="$5"
   
    local start_block=$(( ($first_block + $last_block) / 2 ))

    echo "Starting chain worker with rpc_url=${rpc_url} chain_id=${chain_id} start_block=${start_block}"
    pushd "${VLAYER_HOME}/rust"

    RUST_LOG=${RUST_LOG:-info} \
    ./target/debug/worker \
        --db-path "${db_path}" \
        --rpc-url "${rpc_url}" \
        --chain-id "${chain_id}" \
        --proof-mode "${WORKER_PROOF_ARG}" \
        --confirmations "${CONFIRMATIONS:-1}" \
        --start-block "${start_block}" \
        --max-head-blocks "${MAX_HEAD_BLOCKS:-10}" \
        --max-back-propagation-blocks "${MAX_BACK_PROPAGATION_BLOCKS:-10}" \
        >>"${LOGS_DIR}/chain_worker_${chain_id}.out" 2>&1 &

    local worker_pid=$!
    echo "Chain worker started with PID ${worker_pid}."
    echo "${worker_pid}" >> "${CHAIN_WORKER_PIDS}"

    popd
}

function startup_chain_server() {
    local db_path="$1"

    echo "Starting chain server"
    pushd "${VLAYER_HOME}/rust"

    RUST_LOG=info \
    ./target/debug/chain_server \
        --db-path "${db_path}" \
        >>"${LOGS_DIR}/chain_server.out" &

    CHAIN_SERVER=$!
    
    echo "Chain server started with PID ${CHAIN_SERVER}."
    wait_for_port_and_pid 3001 "${CHAIN_SERVER}" 30m "chain server"

    popd
}

wait_for_chain_worker_sync() {
    local rpc_url="$1"
    local chain_id="$2"
    local first_block="$3"
    local last_block="$4"

    echo "Waiting for chain worker sync... chain_id=${chain_id} first_block=${first_block} last_block=${last_block}"

    for i in `seq 1 20` ; do
        local reply=$(curl -s -X POST 127.0.0.1:3001 \
                  --retry-connrefused \
                  --retry 5 \
                  --retry-delay 0 \
                  --retry-max-time 30 \
                  -H "Content-Type: application/json" \
                  --data '{"jsonrpc": "2.0","id": 0,"method": "v_getSyncStatus","params": ['"${chain_id}"']}'
        )

        local result=$(echo "${reply}" | jq ".result")

        if [[ "${result}" != "null" ]] ; then
            local first_block_synced="true"
            local last_block_synced="true"
            
            if [[ "${first_block}" != "latest" ]] ; then
                first_block_synced=$(echo "${result}" | jq "(.first_block <= ${first_block})")
            fi

            if [[ "${last_block}" != "latest" ]] ; then
                last_block_synced=$(echo "${result}" | jq "(.last_block >= ${last_block})")
            fi

            if [[ "${first_block_synced}" == "true" && "${last_block_synced}" == "true" ]] ; then
                echo "Chain ${chain_id} worker synced"
                break
            fi
        fi

        echo "Syncing ... ${result}"
        sleep 10
    done

    if [[ "${result}" == "null" || "${first_block_synced}" != "true" || "${last_block_synced}" != "true" ]] ; then
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
    WORKER_PROOF_ARG="fake"
    RISC0_DEV_MODE=1
elif [[ "${PROVING_MODE}" == "prod" ]]; then
    SERVER_PROOF_ARG="groth16"
    WORKER_PROOF_ARG="succinct"

    # Check that BONSAI_API_URL and BONSAI_API_KEY are not empty in prod mode
    if [[ -z "$BONSAI_API_URL" || -z "$BONSAI_API_KEY" ]]; then
        echo "Error: BONSAI_API_URL and BONSAI_API_KEY must be set in prod mode."
        usage
    fi
fi

# set external rpc urls
if [[ -z "${QUICKNODE_API_KEY:-}" || -z "${QUICKNODE_ENDPOINT:-}" ]] ; then
    echo QUICKNODE_API_KEY is not configured. Using only local rpc-urls. >&2
    latest_anvil_block=$(get_latest_block "http://localhost:8545")
    CHAIN_WORKER_ARGS=(
        "http://localhost:8545 31337 ${latest_anvil_block} ${latest_anvil_block}"
    )
else
    latest_op_sepolia_block=$(get_latest_block "https://${QUICKNODE_ENDPOINT}.optimism-sepolia.quiknode.pro/${QUICKNODE_API_KEY}")

    if [[ "${EXAMPLE_NAME:-}" == "simple_time_travel" ]]; then
        # Time travel example needs to travel 10 block back
        start_op_sepolia_block=$(( $latest_op_sepolia_block - 10 ))
    else
        start_op_sepolia_block=$latest_op_sepolia_block
    fi

    if [[ "${EXAMPLE_NAME:-}" == "simple_teleport" ]]; then
        CHAIN_WORKER_ARGS=(
            "https://${QUICKNODE_ENDPOINT}.optimism-sepolia.quiknode.pro/${QUICKNODE_API_KEY} 11155420 ${start_op_sepolia_block} ${latest_op_sepolia_block}"
            "https://${QUICKNODE_ENDPOINT}.quiknode.pro/${QUICKNODE_API_KEY} 1 20683110 20683110"
            "https://${QUICKNODE_ENDPOINT}.base-mainnet.quiknode.pro/${QUICKNODE_API_KEY} 8453 19367633 19367633"
            "https://${QUICKNODE_ENDPOINT}.optimism.quiknode.pro/${QUICKNODE_API_KEY} 10 124962954 124962954"
        )
    else
        CHAIN_WORKER_ARGS=(
            "https://${QUICKNODE_ENDPOINT}.optimism-sepolia.quiknode.pro/${QUICKNODE_API_KEY} 11155420 ${start_op_sepolia_block} ${latest_op_sepolia_block}"
        )
    fi

    EXTERNAL_RPC_URLS=(
        "--rpc-url" "1:https://${QUICKNODE_ENDPOINT}.quiknode.pro/${QUICKNODE_API_KEY}"
        "--rpc-url" "8453:https://${QUICKNODE_ENDPOINT}.base-mainnet.quiknode.pro/${QUICKNODE_API_KEY}"
        "--rpc-url" "10:https://${QUICKNODE_ENDPOINT}.optimism.quiknode.pro/${QUICKNODE_API_KEY}"
        "--rpc-url" "11155111:https://${QUICKNODE_ENDPOINT}.ethereum-sepolia.quiknode.pro/${QUICKNODE_API_KEY}"
        "--rpc-url" "84532:https://${QUICKNODE_ENDPOINT}.base-sepolia.quiknode.pro/${QUICKNODE_API_KEY}"
        "--rpc-url" "11155420:https://${QUICKNODE_ENDPOINT}.optimism-sepolia.quiknode.pro/${QUICKNODE_API_KEY}"
    )

fi

# Display the parsed parameters
echo "PROVING_MODE: ${PROVING_MODE}"
echo "BONSAI_API_URL: ${BONSAI_API_URL}"
echo "SERVER_PROOF_ARG: ${SERVER_PROOF_ARG}"
echo "EXTERNAL_RPC_URLS: ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}"
echo

if [[ "${BUILD_BINARIES}" == "1" ]] ; then 
    echo "Building binaries..."
    pushd "${VLAYER_HOME}/rust"
    cargo build --bin vlayer --bin chain_server --bin worker
    popd
    echo
fi

echo "Starting services..."

start_anvil
if [[ "${RUN_CHAIN_SERVICES:-0}" == "1" ]] ; then 
    startup_chain_services "${CHAIN_WORKER_ARGS[@]}"
fi
startup_vlayer "${SERVER_PROOF_ARG}" ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}

echo "Services have been successfully started"
