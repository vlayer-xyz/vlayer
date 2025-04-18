#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-contracts.sh"

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function install_deps {
    echo "::group::Installing dependencies"
    build_react_sdk_with_deps
    
    pushd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    rm -rf node_modules
    bun install --frozen-lockfile
    popd
    echo "::endgroup::Installing dependencies"
}

function run_web_app {
    echo "::group::Running web app"
    pushd ${VLAYER_HOME}/examples/simple-web-proof/vlayer
    bun run web:dev &
    popd
    echo "::endgroup::Running web app"
}

function run_browser_extension {
    echo "::group::Running browser extension"
    pushd ${VLAYER_HOME}/packages/browser-extension
    bun run dev
    popd
    echo "::endgroup::Running browser extension"
}

DOCKER_COMPOSE_SERVICES="anvil-l1 anvil-l2-op wsproxy wsproxy-test-client notary-server"
source ${VLAYER_HOME}/bash/run-services.sh

install_deps
build_core_contracts
build_example_contracts simple-web-proof
run_web_app
run_browser_extension
