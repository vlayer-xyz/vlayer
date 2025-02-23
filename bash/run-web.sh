#!/usr/bin/env bash

set -ueo pipefail
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/run_services/lib.sh"
source "$(dirname "${BASH_SOURCE[0]}")/run_services/config.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"

# Define color codes
BLUE='\033[0;34m'
NC='\033[0m' # No Color

function cleanup {
    pkill -TERM -P $$
}

trap cleanup SIGINT

VLAYER_ENV=${VLAYER_ENV:-dev}
BUILD_BINARIES=${BUILD_BINARIES:-1}
set_proving_mode
BONSAI_API_URL="${BONSAI_API_URL:-https://api.bonsai.xyz/}"
BONSAI_API_KEY="${BONSAI_API_KEY:-}"
SERVER_PROOF_ARG="fake"
EXTERNAL_RPC_URLS=()
DOCKER_COMPOSE_FILE="${VLAYER_HOME}/docker/docker-compose.devnet.yaml"

setup_tmp_dir

# Logging setup
LOGS_DIR="${VLAYER_TMP_DIR}/logs"
mkdir -p "${LOGS_DIR}"
echo "Saving artifacts to: ${VLAYER_TMP_DIR}"

# Chain worker PIDS setup (for cleanup)
CHAIN_WORKER_PIDS="${VLAYER_TMP_DIR}/chain_worker_pids"
touch "${CHAIN_WORKER_PIDS}"

set_proof_mode
set_external_rpc_urls
set_chain_worker_args


function build_js {
    echo -e "${BLUE}Installing dependencies...${NC}"
    pushd ${VLAYER_HOME}
    bun install --frozen-lockfile
    popd
    pushd ${VLAYER_HOME}/packages/sdk
    bun run build --watch &
    popd 

    pushd ${VLAYER_HOME}/packages/sdk-hooks
    bun run build --watch &
    popd
    
    pushd ${VLAYER_HOME}
    bun install --frozen-lockfile
    popd
}

function run_services_in_docker {
    echo -e "${BLUE}Running services in Docker...${NC}"
    cd ${VLAYER_HOME}
    docker-compose -f docker/docker-compose.devnet.yaml up -d anvil-l1 notary-server wsproxy json-server 
}

function build_contracts {
    echo -e "${BLUE}Building contracts...${NC}"
    cd ${VLAYER_HOME}/contracts/vlayer
    forge soldeer install
    forge clean
    forge build
    cd -

    cd ${VLAYER_HOME}/examples/simple-web-proof
    forge clean
    forge build
    cd -

    cd ${VLAYER_HOME}/examples/simple-email-proof
    forge clean
    forge build
    cd -

    cd ${VLAYER_HOME}/bash
    ./build-ts-types.sh
    cd -
}

function run_web_example {
    echo -e "${BLUE}Running web example...${NC}"
    cd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    bun run web:dev &
}

function run_email_example {
    echo -e "${BLUE}Running email example...${NC}"
    cd ${VLAYER_HOME}/examples/simple-email-proof/vlayer
    bun run web:dev &
}

function run_test_web_app {
    echo -e "${BLUE}Running test web app...${NC}"
    cd ${VLAYER_HOME}/packages/test-web-app
    bun run dev &
}

function run_browser_extension {
    echo -e "${BLUE}Running browser extension...${NC}"
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev 
}

function run_app {
    case "$1" in
        web-example)
            run_web_example
            ;;
        email-example)
            run_email_example
            ;;
        test-eweb-app)
            run_test_eweb_app
            ;;
        *)
            echo "Unknown app: $1"
            exit 1
            ;;
    esac
}

if [[ $# -eq 0 ]]; then
    echo "Usage: $0 --app <app-name>"
    exit 1
fi

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --app)
            APP_NAME="$2"
            shift # past argument
            shift # past value
            ;;
        *)
            echo "Unknown option: $key"
            exit 1
            ;;
    esac
done

build_contracts
wait # Ensure build_contracts completes before proceeding

build_js

run_services_in_docker
# our servers running outside docker 
startup_vlayer "${SERVER_PROOF_ARG}" ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}
startup_vdns_server
run_app "$APP_NAME"
sleep 2
run_browser_extension