#!/bin/bash

set -ueo pipefail

cleanup() {
    echo "Cleaning up..."
    
    # Kill Anvil if running
    if [ -n "$ANVIL_PID" ] && ps -p "$ANVIL_PID" > /dev/null; then
        echo "Killing Anvil (PID $ANVIL_PID)..."
        kill "$ANVIL_PID"
    fi

    # Kill Vlayer server if running
    if [ -n "$VLAYER_SERVER_PID" ] && ps -p "$VLAYER_SERVER_PID" > /dev/null; then
        echo "Killing Vlayer server (PID $VLAYER_SERVER_PID)..."
        kill "$VLAYER_SERVER_PID"
    fi
}

trap cleanup EXIT ERR INT

export RISC0_DEV_MODE=true

check_exit_status() {
  if [ $? -ne 0 ]; then
    echo "$1"
    exit 1
  fi
}

wait_for_port() {
    local port=$1
    local timeout=$2
    local service_name=$3
    echo "Waiting for $service_name to be ready on localhost:$port..."

    timeout "$timeout" bash -c "until nc -z localhost $port; do sleep 3; done"
    check_exit_status "Error: Timeout reached. $service_name is not available on localhost:$port."
}

LOGS_DIR="../../../logs"
mkdir -p "$LOGS_DIR"

echo "Starting Anvil"
anvil >"${LOGS_DIR}/anvil.out" 2>"${LOGS_DIR}/anvil.err" &
ANVIL_PID=$!
echo "Anvil started with PID $ANVIL_PID."
wait_for_port 8545 30s "Anvil"

echo "Starting Vlayer REST server"
cd "../../../rust"
LOGS_DIR="../logs"
RUST_LOG=info cargo run --bin vlayer serve >"${LOGS_DIR}/vlayer_serve.out" 2>"${LOGS_DIR}/vlayer_serve.err" &
VLAYER_SERVER_PID=$!
echo "Vlayer server started with PID ${VLAYER_SERVER_PID}."
wait_for_port 3000 20m "Vlayer server"

echo "Deploying smart contract webProver"
cd "../examples/web"
source ../../bash/vlayer-deploy.sh web

echo "Sending v_call request to Vlayer REST server"
(cd ./vlayer/; bun run index.ts)