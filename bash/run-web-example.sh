#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/build-sdk.sh"

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function install_deps {
    echo "::group::Installing dependencies"
    build_react_sdk_with_deps
    
    cd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    rm -rf node_modules
    bun install --frozen-lockfile
    echo "::endgroup::Installing dependencies"
}

function build_example_contracts {
    echo "::group::Building example contracts" 
    cd ${VLAYER_HOME}/examples/simple-web-proof
    forge clean
    forge build
    echo "::endgroup::Building example contracts"
}

function run_web_app {
    echo "::group::Running web app"
    cd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    bun run web:dev &
    echo "::endgroup::Running web app"
}

function run_browser_extension {
    echo "::group::Running browser extension"
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev
    echo "::endgroup::Running browser extension"
}

install_deps
build_example_contracts
run_web_app
run_browser_extension
