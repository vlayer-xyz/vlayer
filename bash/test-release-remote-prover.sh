#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/set-missing-git-config.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"

set -ueo pipefail

VLAYER_RELEASE=${VLAYER_RELEASE:-nightly}

echo "::group::setting git config"
set_missing_git_config
echo "::endgroup::"

echo "::group::foundry installation"
curl -L https://foundry.paradigm.xyz | bash
export PATH="$PATH:$HOME/.config/.foundry/bin"
foundryup
echo "::endgroup::"

echo "::group::vlayer installation"
curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:$HOME/.config/.vlayer/bin"
vlayerup --channel "${VLAYER_RELEASE}"
vlayer --version
echo "::endgroup::"

echo "::group::bun installation"
curl -fsSL https://bun.sh/install | bash -s "bun-v1.2.4"
export PATH="$PATH:~/.bun/bin"
echo "::endgroup::"

echo "::group::risczero installation"
curl -L https://risczero.com/install | bash
export PATH="$PATH:~/.risc0/bin"
export PATH="$PATH:~/.cargo/bin"
rzup install r0vm 2.0.1
rzup install rust 1.85.0
rzup show
echo "::endgroup::"


BUN_NO_FROZEN_LOCKFILE=1
VLAYER_ENV="testnet"
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

build_extension

echo "Starting VDNS server"
docker compose -f ${VLAYER_HOME}/docker/docker-compose.devnet.yaml up -d vdns_server

TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX)
mkdir -p "${TEMP_DIR}/packages/browser-extension"
cp -a "${VLAYER_HOME}/packages/browser-extension/dist" "${TEMP_DIR}/packages/browser-extension"

echo "::group::Installing playwright chromium"
install_chromium
echo "::endgroup::"

# We need to first create a temporary directory and perform operations there.
# After that we copy the results to the VLAYER_HOME directory.
# This is necessary because initializing in the repository directly causes
# `forge init (called by vlayer init)` error because git is already initialized. 
# This is a workaround to avoid this and make all the playwright reports available
# in the VLAYER_HOME directory.

PLAYWRIGHT_FAILED=false

for example in $(get_examples); do
    echo "::group::Initializing vlayer template: ${example}"
    cd "${TEMP_DIR}"
    mkdir -p examples/${example}
    echo "Current directory: $(pwd)"

    VLAYER_TEMP_DIR="examples/${example}"
    cd "${VLAYER_TEMP_DIR}"
 
    vlayer init --template "${example}"
    echo "Current directory: $(pwd)"
    forge build
    vlayer test
    echo "::endgroup::"

    echo "::group::vlayer run prove.ts: ${example}"
    run_prover_script
    echo "::endgroup::"

    echo "::group::vlayer run Playwright test: ${example}"
    if ! run_playwright_tests; then
        echo "Playwright test failed for ${example}..."
        PLAYWRIGHT_FAILED=true
    fi
    echo "::endgroup::"
done

# Copy the TEMP_DIR to the final destination after the loop
echo "Copying from ${TEMP_DIR} to ${VLAYER_HOME}/vlayer-test-release"
mkdir -p "${VLAYER_HOME}/vlayer-test-release"
cp -a "${TEMP_DIR}"/* "${VLAYER_HOME}/vlayer-test-release"

# Exit with 1 if any Playwright test failed
if [ "$PLAYWRIGHT_FAILED" = true ]; then
    exit 1
fi
