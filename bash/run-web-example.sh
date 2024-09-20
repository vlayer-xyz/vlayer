#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function run_services {
    source ${VLAYER_HOME}/bash/run-services.sh 
}

function deploy_contracts {
    cd ${VLAYER_HOME}/examples/web_proof/vlayer
    bun run deploy.ts
}

function run_web_app {
    cd ${VLAYER_HOME}/examples/web_proof/vlayer
    bun run dev &
}

function run_browser_extension {
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev 
}

function install_deps {
    cd ${VLAYER_HOME}/packages && bun install
    cd ${VLAYER_HOME}/examples/web_proof/vlayer && bun install
}

install_deps
run_services
deploy_contracts
run_web_app
run_browser_extension
