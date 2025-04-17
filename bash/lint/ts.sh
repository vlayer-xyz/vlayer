#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

source "$VLAYER_HOME/bash/lib/colors.sh"
source "$VLAYER_HOME/bash/lib/examples.sh"
source "$VLAYER_HOME/bash/lib/build-packages.sh"
source "$VLAYER_HOME/bash/lib/build-contracts.sh"

usage() {
    echo_color YELLOW "Usage: $0 [OPTIONS]"
    echo_color YELLOW "Options:"
    echo_color YELLOW " --help      Display this help message"
    echo_color YELLOW " --fix       Fix linting errors"
}

SKIP_BUILD=false
FIX_FLAG=""

handle_options() {
    while [ $# -gt 0 ]; do
        case $1 in
            --help)
                usage
                exit 0
                ;;
            --fix)
                FIX_FLAG=" --fix"
                ;;
            --skip-build)
                SKIP_BUILD=true
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

handle_options "$@"

if [ "$SKIP_BUILD" = false ]; then
    echo "::group::Building react sdk with deps"
    build_react_sdk_with_deps
    echo "::endgroup::Building react sdk with deps"
fi

echo "::group::Building contracts"
build_contracts
echo "::endgroup::Building contracts"

echo "::group::Running eslint "
pushd "${VLAYER_HOME}"
bun run lint:packages $FIX_FLAG
bun run lint:examples $FIX_FLAG
popd
echo "::endgroup::Running eslint"
