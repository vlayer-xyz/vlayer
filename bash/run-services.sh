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
    if [[ -n "${ANVIL1_PID:-}" ]] && ps -p "$ANVIL1_PID" > /dev/null; then
        echo "Killing anvil 1 (PID $ANVIL1_PID)..."
        kill "$ANVIL1_PID"
    fi
    if [[ -n "${ANVIL2_PID:-}" ]] && ps -p "$ANVIL2_PID" > /dev/null; then
        echo "Killing anvil 2 (PID $ANVIL2_PID)..."
        kill "$ANVIL2_PID"
    fi

    # Kill vlayer server if running
    if [[ -n "${VLAYER_SERVER_PID:-}" ]] && ps -p "$VLAYER_SERVER_PID" > /dev/null; then
        echo "Killing vlayer server (PID $VLAYER_SERVER_PID)..."
        kill "$VLAYER_SERVER_PID"
    fi
}

function startup_anvils(){
    echo "Starting Anvil 1"
    startup_anvil "${LOGS_DIR}/anvil1.out" 8545 ANVIL1_PID

    echo "Starting Anvil 2"
    startup_anvil "${LOGS_DIR}/anvil2.out" 8546 ANVIL2_PID
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
        --rpc-url 100002:http://localhost:8546 \
        --rpc-url 100001:http://localhost:8545 \
        ${external_urls[@]+"${external_urls[@]}"} \
        >"${LOGS_DIR}/vlayer_serve.out" &

    VLAYER_SERVER_PID=$!

    echo "vlayer server started with PID ${VLAYER_SERVER_PID}."
    wait_for_port_and_pid 3000 ${VLAYER_SERVER_PID} 30m "vlayer server"

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
    echo ALCHEMY_API_KEY is not configured. Using using only local rpc-urls. >&2
else 

    EXTERNAL_RPC_URLS=(
        "--rpc-url" "1:https://eth-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}" 
        "--rpc-url" "8453:https://base-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
        "--rpc-url" "10:https://opt-mainnet.g.alchemy.com/v2/${ALCHEMY_API_KEY}"
    )

fi

# Display the parsed parameters
echo "PROVING_MODE: ${PROVING_MODE}"
echo "BONSAI_API_URL: ${BONSAI_API_URL}"
echo "SERVER_PROOF_ARG: ${SERVER_PROOF_ARG}"


echo
echo "Starting services..."

startup_anvils
startup_vlayer "${SERVER_PROOF_ARG}" ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}

echo "Services has been succesfully started..."
