source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib.sh"

function prepare-to-lint() {
    bun install --frozen-lockfile

    build_contracts

    build_sdk

    build_sdk_hooks
}