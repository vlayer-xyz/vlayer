#!/usr/bin/env bash

set -ueo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/common.sh"

ANVIL_PORT=8546

function build_contracts() {
    (
    cd contracts
    forge soldeer install
    forge clean
    forge build
    )
}

function deploy_address() {
    (
    cd packages
    bun init > /dev/null
    bun install --frozen-lockfile > /dev/null
    bun run ./src/tests/deployVerifier.ts
    )
}

####### SETUP
startup_anvil "/dev/null" ${ANVIL_PORT} ANVIL_PID
trap "kill ${ANVIL_PID}" EXIT
build_contracts

####### TEST

ADDRESS=$(deploy_address)
echo "contract deployed with address ${ADDRESS}"

GUEST_ID_SOL=$(cast call ${ADDRESS} "guest_id()(bytes32)" --rpc-url 127.0.0.1:${ANVIL_PORT})
check_exit_status "Error: Failed to call contract method"

GUEST_ID="0x$(./rust/target/release/vlayer --version | awk '/CALL_GUEST_ID:/ {print $2}')"

if [[ ${GUEST_ID_SOL} == ${GUEST_ID} ]]; then
    echo "OK: GUEST_ID matches"
else
    echo -e "ERROR: GUEST_ID does not match: \
    \nGUEST_ID from deployed contract = ${GUEST_ID_SOL} \
    \nGUEST_ID from vlayer --version = ${GUEST_ID}" >&2

    exit 1
fi
