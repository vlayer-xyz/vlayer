#!/usr/bin/env bash

set -ueo pipefail

function run_services() {
    pushd "${VLAYER_HOME}/rust/scripts/run_services"
    cargo run --release -- "$@" &
    SERVICES_PID=$!
    popd
}

function cleanup() {
    if [[ -n "${SERVICES_PID:-}" ]] && ps -p "$SERVICES_PID" > /dev/null; then
        echo "Killing services (PID $SERVICES_PID)..."
        kill "$SERVICES_PID"
    fi
}

trap cleanup EXIT ERR INT
run_services "$@"
echo "Services have been successfully started"
