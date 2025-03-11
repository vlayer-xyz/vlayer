#!/usr/bin/env bash

set -ueo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/lib/colors.sh"

usage() {
    echo_color YELLOW "Usage: $0 [OPTIONS]"
    echo_color YELLOW "Options:"
    echo_color YELLOW " --help      Display this help message"
    echo_color YELLOW " --fix       Fix linting errors"
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
                echo_color RED "Invalid option: $1" >&2
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
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"

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
