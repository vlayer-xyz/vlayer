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
curl -fsSL https://bun.sh/install | bash
export PATH="$PATH:~/.bun/bin"
echo '::endgroup::'

git config --global user.email "test@example.com"
git config --global user.name "Github Runner"

echo '::group::vlayer template initialization'
VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
cd ${VLAYER_TEMP_DIR}

vlayer init --template web-proof
forge build

cd vlayer
bun install
echo '::endgroup::'

echo '::group::vlayer run prove.ts'
bun run prove.ts
echo '::endgroup::'
