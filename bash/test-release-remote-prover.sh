#!/usr/bin/env bash

set -ueo pipefail

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
cargo binstall -y cargo-risczero@1.2.1-rc.1
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

    echo "::group::Initializing vlayer template: ${example_name}"
    VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
    cd ${VLAYER_TEMP_DIR}

    vlayer init --template "${example_name}"
    forge build
    vlayer test

    cd vlayer
    # Sadly, bun's manifest caching is so unstable, it causes random `bun install` freezes.
    # To circumvent that for the time being, we disable all caching.
    # https://github.com/oven-sh/bun/issues/5831
    bun install --no-cache
    echo '::endgroup::'

    echo "::group::vlayer run prove.ts: ${example_name}"
    bun run prove:testnet
    echo '::endgroup::'
done
