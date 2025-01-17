#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function install_deps {
    cd ${VLAYER_HOME}
    bun install --frozen-lockfile

    cd ${VLAYER_HOME}/packages/sdk
    bun run build
    
    cd ${VLAYER_HOME}/examples/simple_web_proof/vlayer
    rm -rf node_modules
    bun install --frozen-lockfile
}

function run_services {
    source ${VLAYER_HOME}/bash/run-services.sh 
}

function build_example_contracts {
    cd ${VLAYER_HOME}/examples/simple_web_proof
    forge build
}

function run_web_app {
    cd ${VLAYER_HOME}/examples/simple_web_proof/vlayer
    bun run web:dev &
}

function run_browser_extension {
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev 
}

install_deps
run_services
build_example_contracts
run_web_app
run_browser_extension
