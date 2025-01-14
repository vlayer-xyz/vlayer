#!/usr/bin/env bash

set -ueo pipefail

echo '::group::vlayer installation'
curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:$HOME/.config/.vlayer/bin"
vlayerup
echo '::endgroup::'

echo '::group::bun installation'
curl -fsSL https://bun.sh/install | bash
export PATH="$PATH:~/.bun/bin"
echo '::endgroup::'

echo '::group::risczero installation'
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
cargo binstall -y cargo-risczero@1.2.0
cargo risczero install
echo '::endgroup::'

echo '::group::Playwright browser installation'
bunx playwright install --with-deps chromium
echo '::endgroup::'

git config --global user.email "test@example.com"
git config --global user.name "Github Runner"

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
    example_name=$(basename "${example}"  | tr '_' '-')

    # We're restarting anvil because some examples rely on a clean chain state.
    echo "Restarting anvil"
    docker compose -f ${VLAYER_HOME}/docker/docker-compose.devnet.yaml restart anvil-a

    echo "::group::Initializing vlayer template: ${example_name}"
    VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
    cd ${VLAYER_TEMP_DIR}

    vlayer init --template "${example_name}"
    forge build
    vlayer test

    cd vlayer
    bun install --verbose
    echo '::endgroup::'

    echo "::group::vlayer run prove.ts: ${example_name}"
    bun run prove:dev
    echo '::endgroup::'
done
