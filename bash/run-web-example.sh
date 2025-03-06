#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/build-sdk.sh"

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function install_deps {
    build_react_sdk_with_deps
    
    cd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    rm -rf node_modules
    bun install --frozen-lockfile
}

function build_example_contracts {    
    cd ${VLAYER_HOME}/examples/simple-web-proof
    forge clean
    forge build
}

function run_web_app {
    cd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    bun run web:dev &
}

function run_browser_extension {
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev
}

install_deps
build_example_contracts
run_web_app
run_browser_extension
