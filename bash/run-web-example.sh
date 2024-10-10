#!/usr/bin/env bash

echo 'running'
set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
MODE="headed"
while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        --mode)
        MODE="$2"
        shift
        shift
        ;;
        *)
        echo "Unknown option $1"
        exit 1
        ;;
    esac
done

function calculate_extension_id {
    cd ${VLAYER_HOME}/examples/web_proof/vlayer
    bun run calcExtensionId.ts
}

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

function build_browser_extension {
  cd ${VLAYER_HOME}/packages/browser-extension
  bun run build &
}

function run_browser_extension {
    cd ${VLAYER_HOME}/packages/browser-extension
    bun run dev
}

function install_deps {
    cd ${VLAYER_HOME}/packages && bun install --frozen-lock-file
    cd ${VLAYER_HOME}/examples/web_proof/vlayer && bun install --frozen-lock-file
}

if [ "$MODE" = "headless" ]; then
    install_deps
    build_browser_extension
    run_web_app
    wait
else
    install_deps
    run_services
    deploy_contracts
    calculate_extension_id
    run_web_app
    run_browser_extension
fi
