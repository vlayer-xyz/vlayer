#!/usr/bin/env bash

set -ueo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/lib/colors.sh"

usage() {
    echo -e "${YELLOW}Usage: $0 [OPTIONS]${NC}"
    echo -e "${YELLOW}Options:${NC}"
    echo -e "${YELLOW} --help      Display this help message${NC}"
    echo -e "${YELLOW} --fix       Fix linting errors${NC}"
}

handle_options() {
    while [ $# -gt 0 ]; do
        case $1 in
            --help)
                usage
                exit 0
                ;;
            --fix)
                FIX_FLAG=" --fix"
                FIX_OPTION=":fix"
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

FIX_FLAG=""
FIX_OPTION=""

handle_options "$@"

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build_sdk.sh"

build_react_sdk_with_deps

echo "::group::Running eslint for examples"
for example in $(get_examples); do (
    echo "Running eslint${FIX_FLAG} for: ${example}"
    pushd "$VLAYER_HOME/examples/$example/vlayer"
    bun run eslint .$FIX_FLAG
    popd
) done
echo '::endgroup::Running eslint for examples'

echo "::group::Running eslint for: $VLAYER_HOME/packages"
pushd "${VLAYER_HOME}/packages"
bun run lint$FIX_OPTION
popd
echo "::endgroup::Running eslint for: $VLAYER_HOME/packages"
