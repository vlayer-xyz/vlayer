set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

function run_services {
    source ${VLAYER_HOME}/bash/run-services.sh &
}

function deploy_contracts {
    cd ${VLAYER_HOME}/examples/web_proof/vlayer
    bun run deploy.ts
}

function run_web_app {
    cd ${VLAYER_HOME}/examples/web_proof/vlayer
    bun run dev &
}

function run_browser_plugin {
    cd ${VLAYER_HOME}/packages/browser-plugin
    bun run dev &
}

run_services
deploy_contracts
run_web_app
run_browser_plugin