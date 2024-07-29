#!/usr/bin/env bash

set -ueo pipefail

if [[ -z "${VLAYER_TMP_DIR:-}" ]] ; then
    VLAYER_TMP_DIR=$(mktemp -d -t vlayer-$(basename $0).XXXXX)
else
    VLAYER_TMP_DIR=$(realpath "${VLAYER_TMP_DIR}")
fi

LOGS_DIR="${VLAYER_TMP_DIR}/logs"
VLAYER_HOME=$(git rev-parse --show-toplevel)

mkdir -p "${LOGS_DIR}"
echo "Saving artifacts to: ${VLAYER_TMP_DIR}"

function cleanup() {
    echo "Cleaning up..."
    
    # Kill Anvil if running
    if [[ -n "${ANVIL_PID:-}" ]] && ps -p "$ANVIL_PID" > /dev/null; then
        echo "Killing anvil (PID $ANVIL_PID)..."
        kill "$ANVIL_PID"
    fi

    # Kill vlayer server if running
    if [[ -n "${VLAYER_SERVER_PID:-}" ]] && ps -p "$VLAYER_SERVER_PID" > /dev/null; then
        echo "Killing vlayer server (PID $VLAYER_SERVER_PID)..."
        kill "$VLAYER_SERVER_PID"
    fi
}

check_exit_status() {
  if [ $? -ne 0 ]; then
    echo "$1"
    exit 1
  fi
}

wait_for_port_and_pid() {
    local port=$1
    local pid=$2
    local timeout=$3
    local service_name=$4
    echo "Waiting for ${service_name} to be ready on localhost:${port}..."

    # wait until port is open and the expected pid is alive
    # if the port is open, but pid is not alive, exit 
    timeout >/dev/null 2>&1 --preserve-status --foreground --kill-after=5s "${timeout}" bash -c \
        "sleep 3 ;  while ! (nc -z localhost ${port}) && ps -p $pid ; do  sleep 3;  done ; if ! (ps -p $pid) ; then exit 1 ; fi"
    
    check_exit_status "Error: Timeout reached. ${service_name} is not available on localhost:${port}."
}

function startup_anvil(){
    echo "Starting Anvil"
    anvil >"${LOGS_DIR}/anvil.out" &
    ANVIL_PID=$!
    echo "Anvil started with PID ${ANVIL_PID}."
    wait_for_port_and_pid 8545 ${ANVIL_PID} 30s "Anvil"
}

function startup_vlayer(){
    echo "Starting vlayer REST server"
    (
        cd "${VLAYER_HOME}/rust"

        RUST_LOG=info \
        RISC0_DEV_MODE="${RISC0_DEV_MODE}" \
        BONSAI_API_URL="${BONSAI_API_URL}" \
        BONSAI_API_KEY="${BONSAI_API_KEY}" \
        cargo run --bin vlayer serve >"${LOGS_DIR}/vlayer_serve.out" & 

        VLAYER_SERVER_PID=$!

        echo "vlayer server started with PID ${VLAYER_SERVER_PID}."
        wait_for_port_and_pid 3000 ${VLAYER_SERVER_PID} 30m "vlayer server"

    )
}

trap cleanup EXIT ERR INT

# Default values
PROVING_MODE=${PROVING_MODE:-dev}
RISC0_DEV_MODE=""
BONSAI_API_URL="${BONSAI_API_URL:-https://api.bonsai.xyz/}"
BONSAI_API_KEY="${BONSAI_API_KEY:-}"


# Set the RISC0_DEV_MODE environment variable based on the mode
if [[ "${PROVING_MODE}" == "dev" ]]; then
    RISC0_DEV_MODE=true
elif [[ "${PROVING_MODE}" == "prod" ]]; then
    RISC0_DEV_MODE=false

    # Check that BONSAI_API_URL and BONSAI_API_KEY are not empty in prod mode
    if [[ -z "$BONSAI_API_URL" || -z "$BONSAI_API_KEY" ]]; then
        echo "Error: BONSAI_API_URL and BONSAI_API_KEY must be set in prod mode."
        usage
    fi
fi

# Display the parsed parameters
echo "PROVING_MODE: ${PROVING_MODE}"
echo "RISC0_DEV_MODE: ${RISC0_DEV_MODE}"
echo "BONSAI_API_URL: ${BONSAI_API_URL}"


echo
echo "Starting services..."

startup_anvil
startup_vlayer

echo "Services has been succesfully started..."
