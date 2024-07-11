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

# Default values
MODE=""
CALLER=""
DATA=""
EXAMPLE=""
RISC0_DEV_MODE=""
export BONSAI_API_URL="${BONSAI_API_URL:-https://api.bonsai.xyz/}"
export BONSAI_API_KEY="${BONSAI_API_KEY:-}"

# Usage function to display help for the hapless user
usage() {
    echo "Usage: $0 --mode dev|prod --caller CALLER --data DATA --example EXAMPLE"
    echo
    echo "  --mode        Set the mode (dev or prod)"
    echo "  --caller      Set the caller"
    echo "  --data        Set the data"
    echo "  --example     Set the example"
    echo "  --bonsai-api-key  Set the Bonsai API key (or use BONSAI_API_KEY environment variable)"
    echo
    echo "Example:"
    echo "  $0 --mode dev --caller 0xf39f... --data 0xca... --example simple"
    exit 1
}

# Parse the command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --mode)
            if [[ "$2" == "dev" || "$2" == "prod" ]]; then
                MODE="$2"
                shift 2
            else
                echo "Error: Invalid mode. Must be 'dev' or 'prod'."
                usage
            fi
            ;;
        --caller)
            CALLER="$2"
            shift 2
            ;;
        --data)
            DATA="$2"
            shift 2
            ;;
        --example)
            EXAMPLE="$2"
            shift 2
            ;;
        --bonsai-api-key)
            export BONSAI_API_KEY="$2"
            shift 2
            ;;
        *)
            echo "Error: Invalid parameter."
            usage
            ;;
    esac
done

# Check if all required parameters are provided
if [[ -z "$MODE" || -z "$CALLER" || -z "$DATA" || -z "$EXAMPLE" ]]; then
    echo "Error: Missing required parameters."
    usage
fi

# Set the RISC0_DEV_MODE environment variable based on the mode
if [[ "$MODE" == "dev" ]]; then
    export RISC0_DEV_MODE=true
elif [[ "$MODE" == "prod" ]]; then
    export RISC0_DEV_MODE=false

    # Check that BONSAI_API_URL and BONSAI_API_KEY are not empty in prod mode
    if [[ -z "$BONSAI_API_URL" || -z "$BONSAI_API_KEY" ]]; then
        echo "Error: BONSAI_API_URL and BONSAI_API_KEY must be set in prod mode."
        usage
    fi
fi

# Display the parsed parameters
echo "Mode: $MODE"
echo "Caller: $CALLER"
echo "Data: $DATA"
echo "Example: $EXAMPLE"
echo "RISC0_DEV_MODE: $RISC0_DEV_MODE"
echo "BONSAI_API_URL: $BONSAI_API_URL"

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

VLAYER_HOME=$(pwd)
LOGS_DIR="${VLAYER_HOME}/logs"
mkdir -p "$LOGS_DIR"

echo "Starting Anvil"
anvil >"${LOGS_DIR}/anvil.out" 2>"${LOGS_DIR}/anvil.err" &
ANVIL_PID=$!
echo "Anvil started with PID $ANVIL_PID."
wait_for_port 8545 30s "Anvil"

echo "Starting Vlayer REST server"
cd "${VLAYER_HOME}/rust"
RUST_LOG=info cargo run --bin vlayer serve >"${LOGS_DIR}/vlayer_serve.out" 2>"${LOGS_DIR}/vlayer_serve.err" &
VLAYER_SERVER_PID=$!
echo "Vlayer server started with PID ${VLAYER_SERVER_PID}."
wait_for_port 3000 20m "Vlayer server"

echo "Deploying Simple and SimpleVerification smart contracts"
cd "${VLAYER_HOME}/examples/${EXAMPLE}"
source ../../bash/vlayer-deploy.sh

echo "Sending v_call request to Vlayer REST server"
v_call_request_json=$(cat <<EOF
{
    "method": "v_call",
    "params": [
        {"caller": "${CALLER}", "to": "${SIMPLE_PROVER_ADDRESS}", "data": "${DATA}"},
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
