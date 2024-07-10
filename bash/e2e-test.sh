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

trap cleanup EXIT ERR

VLAYER_HOME=$(pwd)
LOGS_DIR="${VLAYER_HOME}/logs"
mkdir -p "$LOGS_DIR"

echo "Starting Anvil"
anvil >"${LOGS_DIR}/anvil.out" 2>"${LOGS_DIR}/anvil.err" &
ANVIL_PID=$!
echo "Anvil started with PID $ANVIL_PID."
wait_for_port 8545 30s "Anvil"

echo "Starting Vlayer REST server"
cd "$VLAYER_HOME/rust"
RISC0_DEV_MODE=1 RUST_LOG=info cargo run --bin vlayer serve >"${LOGS_DIR}/vlayer_serve.out" 2>"${LOGS_DIR}/vlayer_serve.err" &
VLAYER_SERVER_PID=$!
echo "Vlayer server started with PID $VLAYER_SERVER_PID."
wait_for_port 3000 20m "Vlayer server"

echo "Deploying Simple and SimpleVerification smart contracts"
cd "$VLAYER_HOME/examples/simple"
source ../../bash/vlayer-deploy.sh

echo "Sending v_call request to Vlayer REST server"
v_call_request_json=$(cat <<EOF
{
    "method": "v_call",
    "params": [
        {"caller": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266", "to": "${SIMPLE_PROVER_ADDRESS}", "data": "0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002"},
        {"block_no": ${BLOCK_NO}, "chain_id": 11155111}
    ],
    "id": 1,
    "jsonrpc": "2.0"
}
EOF
)

curl -X POST http://127.0.0.1:3000/ -H "Content-Type: application/json" -d "$v_call_request_json"

# TODO:
# send proof to verification smart contract
