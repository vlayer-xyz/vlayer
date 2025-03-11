#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/set-missing-git-config.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"

set -ueo pipefail

echo '::group::setting git config'
set_missing_git_config
echo '::endgroup::'

echo '::group::foundry installation'
curl -L https://foundry.paradigm.xyz | bash
export PATH="$PATH:$HOME/.config/.foundry/bin"
foundryup
echo '::endgroup::'

echo '::group::vlayer installation'
curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:$HOME/.config/.vlayer/bin"
vlayerup
echo '::endgroup::'

echo '::group::bun installation'
curl -fsSL https://bun.sh/install | bash -s "bun-v1.2.4"
export PATH="$PATH:~/.bun/bin"
echo '::endgroup::'

echo '::group::risczero installation'
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall -y cargo-risczero@1.2.4
cargo risczero install
echo '::endgroup::'


BUN_NO_FROZEN_LOCKFILE=1
VLAYER_ENV="testnet"
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

echo '::group::Build extension'
cd "$VLAYER_HOME/packages/browser-extension"
bun install --frozen-lockfile
bun run build
echo '::endgroup::'

echo "Starting VDNS server"
docker compose -f ${VLAYER_HOME}/docker/docker-compose.devnet.yaml up -d vdns_server

TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX)
mkdir -p "${TEMP_DIR}/packages/browser-extension"
cp -a "${VLAYER_HOME}/packages/browser-extension/dist" "${TEMP_DIR}/packages/browser-extension"

echo '::group::Installing playwright chromium'
install_chromium
echo '::endgroup::'

# We need to first create a temporary directory and perform operations there.
# After that we copy the results to the VLAYER_HOME directory.
# This is necessary because initializing in the repository directly causes
# `forge init (called by vlayer init)` error because git is already initialized. 
# This is a workaround to avoid this and make all the playwright reports available
# in the VLAYER_HOME directory.

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
    run_playwright_tests
    echo "::endgroup::"
done

# Copy the TEMP_DIR to the final destination after the loop
mkdir -p "${VLAYER_HOME}/vlayer-test-release"
cp -a "${TEMP_DIR}"/* "${VLAYER_HOME}/vlayer-test-release"
echo "Copying from ${TEMP_DIR} to ${VLAYER_HOME}/vlayer-test-release"
find "${VLAYER_HOME}/vlayer-test-release" -name "playwright-report"
