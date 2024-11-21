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

function cleanup() {
    echo "Cleaning up..."
    
    # Kill Anvil if running
    if [[ -n "${ANVIL_PID:-}" ]] && ps -p "$ANVIL_PID" > /dev/null; then
        echo "Killing anvil (PID $ANVIL_PID)..."
        kill "$ANVIL_PID"
    fi

    # Kill chain worker if running
    if [[ -n "${CHAIN_WORKER_PID:-}" ]] && ps -p "$CHAIN_WORKER_PID" > /dev/null; then
        echo "Killing chain worker (PID $CHAIN_WORKER_PID)..."
        kill "${CHAIN_WORKER_PID}"
    fi

    # Kill chain server if running
    if [[ -n "${CHAIN_SERVER_PID:-}" ]] && ps -p "${CHAIN_SERVER_PID}" > /dev/null; then
        echo "Killing chain server (PID ${CHAIN_SERVER_PID})..."
        kill "${CHAIN_SERVER_PID}"
    fi

    # Kill vlayer server if running
    if [[ -n "${VLAYER_SERVER_PID:-}" ]] && ps -p "$VLAYER_SERVER_PID" > /dev/null; then
        echo "Killing vlayer server (PID $VLAYER_SERVER_PID)..."
        kill "$VLAYER_SERVER_PID"
    fi
}

function start_anvil(){
    echo "Starting Anvil "
    startup_anvil "${LOGS_DIR}/anvil.out" 8545 ANVIL_PID
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
        --rpc-url 31337:http://localhost:8545 \
        ${external_urls[@]+"${external_urls[@]}"} \
        >"${LOGS_DIR}/vlayer_serve.out" &

    VLAYER_SERVER_PID=$!

    echo "vlayer server started with PID ${VLAYER_SERVER_PID}."
    wait_for_port_and_pid 3000 ${VLAYER_SERVER_PID} 30m "vlayer server"

    popd
}

function startup_chain_services() {
    db_path="${VLAYER_TMP_DIR}/chain_db"
    startup_chain_worker ${db_path} 
    startup_chain_server ${db_path}
}

function startup_chain_worker() {
    db_path="$1"
    echo "Starting chain worker"
    pushd "${VLAYER_HOME}/rust"

    RUST_LOG=info \
    CONFIRMATIONS=0 \
    MAX_BACK_PROPAGATION_BLOCKS=20 \
    MAX_HEAD_BLOCKS=10 \
    RISC0_DEV_MODE=1 \
    RPC_URL=${CHAIN_WORKER_RPC_URL} \
    cargo run --bin worker \
        --db-path "${db_path}" \
        >"${LOGS_DIR}/chain_worker.out" &

    CHAIN_WORKER_PID=$!

    echo "Chain worker started with PID ${CHAIN_WORKER_PID}."

    popd
}

function startup_chain_server() {
    db_path="$1"
    echo "Starting chain server"
    pushd "${VLAYER_HOME}/rust"

    RUST_LOG=info \
    cargo run --bin chain_server \
        --db-path "${db_path}" \
        >"${LOGS_DIR}/chain_server.out" &

    CHAIN_SERVER_PID=$!
    
    echo "Chain server started with PID ${CHAIN_SERVER_PID}."


    echo "Waiting for chain worker sync..."

    for i in `seq 1 10` ; do
        FIRST_BLOCK_MAX=17915294
        LAST_BLOCK_MIN=17985294

        result=$(curl -s -X POST 127.0.0.1:3001 \
                  --retry-connrefused \
                  --retry 5 \
                  --retry-delay 0 \
                  --retry-max-time 30 \
                  -H "Content-Type: application/json" \
                  --data '{"jsonrpc": "2.0","id": 0,"method": "v_sync_status","params": [1]}' \
                  | jq "(.result.first_block <= ${FIRST_BLOCK_MAX}) and (.result.last_block >= ${LAST_BLOCK_MIN})"
        )

        if [[ "${result}" == "true" ]] ; then
            echo "Worker synced"
            break
        fi
        echo "Syncing ..."
        sleep 1
    done

    if [[ "${result}" != "true" ]] ; then
        echo "Failed to sync chain worker" >&2
        exit 1 
    fi



    popd
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
else 
    RUN_CHAIN_SERVICES="${RUN_CHAIN_SERVICES:-1}"
    CHAIN_WORKER_RPC_URL="https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"

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
if [[ "${RUN_CHAIN_SERVICES:-0}" == "1" ]] ; then startup_chain_services ; fi
startup_vlayer "${SERVER_PROOF_ARG}" ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}

echo "Services has been succesfully started..."
