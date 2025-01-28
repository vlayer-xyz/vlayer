#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/set-missing-git-config.sh"

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
curl -fsSL https://bun.sh/install | bash -s "bun-v1.1.34"
export PATH="$PATH:~/.bun/bin"
echo '::endgroup::'

echo '::group::risczero installation'
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall -y cargo-risczero@1.2.1
cargo risczero install
echo '::endgroup::'


BUN_NO_FROZEN_LOCKFILE=1
VLAYER_ENV="testnet"
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

echo "Starting VDNS server"
docker compose -f ${VLAYER_HOME}/docker/docker-compose.devnet.yaml up -d vdns_server

for example in $(get_examples); do

    echo "::group::Initializing vlayer template: ${example}"
    VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
    cd ${VLAYER_TEMP_DIR}

    vlayer init --template "${example}"
    forge build
    vlayer test
    echo '::endgroup::'

    echo "::group::vlayer run prove.ts: ${example}"
    run_prover_script
    echo '::endgroup::'

    echo "::group::vlayer run Playwright test: ${example}"
    USE_PROD_WEB_SERVER=true run_playwright_tests
    echo '::endgroup::'
done
