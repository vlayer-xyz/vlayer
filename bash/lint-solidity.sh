#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
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
                FIX_FLAG="-fix"
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

handle_options "$@"

bun install --frozen-lockfile

echo "::group::Runing solhint for examples"
for example in $(get_examples); do (
    echo "::group::Running solhint for: ${example}"
    pushd "$VLAYER_HOME/examples/$example/vlayer"
    bun run lint$FIX_FLAG:solidity --max-warnings 0
    popd
    echo "::endgroup::Running solhint for: ${example}"
) done
echo "::endgroup::Runing solhint for examples"

echo "::group::Running solhint for contracts directory"
pushd "$VLAYER_HOME/contracts/vlayer"
bun run lint$FIX_FLAG:solidity --max-warnings 0
popd

pushd "$VLAYER_HOME/contracts/fixtures"
bun run lint$FIX_FLAG:solidity --max-warnings 0
popd
echo "::endgroup::Running solhint for contracts directory"