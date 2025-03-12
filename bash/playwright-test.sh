#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/colors.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-contracts.sh"

usage() {
    echo_color YELLOW "Usage: $0 [OPTIONS]"
    echo_color YELLOW "Options:"
    echo_color YELLOW " --help      Display this help message"
    echo_color YELLOW " --headed    Run tests in headed mode (with browser visible)"
}

handle_options() {
    headed_mode=false
    while [ $# -gt 0 ]; do
        case $1 in
            --help)
                usage
                exit 0
                ;;
            --headed)
                headed_mode=true
                ;;
            *)
                echo_color RED "Invalid option: $1" >&2
                usage
                exit 1
                ;;
        esac
        shift
    done
}

# Set default mode
headed_mode=false

# Parse command line arguments
handle_options "$@"

# Main script execution
set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
VLAYER_ENV=${VLAYER_ENV:-dev}

echo_color BLUE "Run services"
source ${VLAYER_HOME}/bash/run-services.sh

echo_color BLUE "Mock ImageId.sol"
source ${VLAYER_HOME}/bash/mock-imageid.sh

build_contracts

echo_color BLUE "Run playwright tests"
pushd ${VLAYER_HOME}/packages
if [ "$headed_mode" = true ]; then
    echo_color BLUE "Running in headed mode"
    VLAYER_ENV=${VLAYER_ENV} bun run test-web:headed
else
    echo_color BLUE "Running in headless mode"
    VLAYER_ENV=${VLAYER_ENV} bun run test-web:headless
fi
popd
