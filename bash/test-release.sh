#!/usr/bin/env bash

set -ueo pipefail

if [ -z "${VLAYER_ENV:-}" ]; then
    echo "Error: VLAYER_ENV is not set."
    exit 1
fi

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

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(find ${VLAYER_HOME}/examples -type d -maxdepth 1 -mindepth 1) ; do
    example_name=$(basename "${example}"  | tr '_' '-')

    if [ ! -f "${example}/vlayer/.env.${VLAYER_ENV}" ]; then
        echo "Skipping ${example_name} as .env.${VLAYER_ENV} file is not defined"
        continue
    fi

    echo "::group::Initializing vlayer template: ${example_name}"
    VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
    cd ${VLAYER_TEMP_DIR}

    vlayer init --template "${example_name}"
    forge build
    vlayer test

    cd vlayer
    bun install
    echo '::endgroup::'

    echo "::group::vlayer run prove.ts: ${example_name}"
    bun run prove.ts
    echo '::endgroup::'
done
