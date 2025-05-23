#!/usr/bin/env bash

set -ueo pipefail

# Imports
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/run_services/lib.sh"
source "$(dirname "${BASH_SOURCE[0]}")/run_services/config.sh"
source "$(dirname "${BASH_SOURCE[0]}")/run_services/cleanup.sh"

# Cleanup setup
trap cleanup EXIT ERR INT

# Default values
VLAYER_ENV=${VLAYER_ENV:-dev}
JWT_AUTH=${JWT_AUTH:-off}
BUILD_SERVICES=${BUILD_SERVICES:-1}
set_proving_mode
BONSAI_API_URL="${BONSAI_API_URL:-https://api.bonsai.xyz/}"
BONSAI_API_KEY="${BONSAI_API_KEY:-}"
SERVER_PROOF_ARG="fake"
EXTERNAL_RPC_URLS=()
DOCKER_COMPOSE_FILES=("-f" "${VLAYER_HOME}/docker/docker-compose.devnet.yaml")
DOCKER_COMPOSE_SERVICES="${DOCKER_COMPOSE_SERVICES:-anvil-l1 anvil-l2-op}"

if [[ "${JWT_AUTH}" == "on" ]]; then
  DOCKER_COMPOSE_FILES+=("-f" "${VLAYER_HOME}/docker/docker-compose.jwt.yaml")
fi

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

# Display the parsed parameters
echo "JWT_AUTH: ${JWT_AUTH}"
echo "PROVING_MODE: ${PROVING_MODE}"
echo "BONSAI_API_URL: ${BONSAI_API_URL}"
echo "SERVER_PROOF_ARG: ${SERVER_PROOF_ARG}"
echo "CHAIN_WORKER_ARGS: ${CHAIN_WORKER_ARGS[@]+"${CHAIN_WORKER_ARGS[@]}"}"
echo "EXTERNAL_RPC_URLS: ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}"
echo "DOCKER_COMPOSE_FILES ${DOCKER_COMPOSE_FILES[@]}"
echo

ensure_services_built

echo "Starting services..."

if [[ $VLAYER_ENV == "dev" ]]; then
    docker compose ${DOCKER_COMPOSE_FILES[@]} up -d --build --force-recreate $DOCKER_COMPOSE_SERVICES
fi

if [[ -n "${EXTERNAL_CHAIN_SERVICE_URL:-}" ]]; then
  echo "Using external chain service at ${EXTERNAL_CHAIN_SERVICE_URL}"
else
  if [[ ${#CHAIN_WORKER_ARGS[@]} -gt 0 ]]; then
    startup_chain_services "${CHAIN_WORKER_ARGS[@]+"${CHAIN_WORKER_ARGS[@]}"}"
  fi
fi
startup_vlayer "${SERVER_PROOF_ARG}" ${EXTERNAL_RPC_URLS[@]+"${EXTERNAL_RPC_URLS[@]}"}

startup_vdns_server

echo "Services have been successfully started"
