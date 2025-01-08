#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function run_services {
    source ${VLAYER_HOME}/bash/run-services.sh 
}

function run_web_app {
    cd ${VLAYER_HOME}/examples/simple_web_proof/vlayer
    bun run web:dev &
}

function run_browser_extension {
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev 
}

function install_deps {
    cd ${VLAYER_HOME}/packages && bun install --frozen-lockfile
    cd ${VLAYER_HOME}/examples/simple_web_proof/vlayer && bun install --frozen-lockfile
}

install_deps
run_services
run_web_app
run_browser_extension
