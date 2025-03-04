#!/bin/bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/colors.sh"

usage() {
    echo -e "${YELLOW}Usage: $0 [OPTIONS]${NC}"
    echo -e "${YELLOW}Options:${NC}"
    echo -e "${YELLOW} --help      Display this help message${NC}"
    echo -e "${YELLOW} --headed    Run tests in headed mode (with browser visible)${NC}"
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
                echo -e "${RED}Invalid option: $1${NC}" >&2
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

echo -e "${BLUE}Run services${NC}"
source ${VLAYER_HOME}/bash/run-services.sh

echo -e "${BLUE}Mock ImageId.sol${NC}"
source ${VLAYER_HOME}/bash/mock-imageid.sh

echo -e "${BLUE}Run Forge build vlayer${NC}"
pushd ${VLAYER_HOME}/contracts/vlayer
forge soldeer install
forge clean
forge build
popd

echo -e "${BLUE}Run Forge build fixtures${NC}"
pushd ${VLAYER_HOME}/contracts/fixtures
forge soldeer install
forge clean
forge build
popd

echo -e "${BLUE}Run playwright tests${NC}"
pushd ${VLAYER_HOME}/packages
if [ "$headed_mode" = true ]; then
    echo -e "${BLUE}Running in headed mode${NC}"
    VLAYER_ENV=${VLAYER_ENV} bun run test:headed
else
    echo -e "${BLUE}Running in headless mode${NC}"
    VLAYER_ENV=${VLAYER_ENV} bun run test:headless
fi
popd
