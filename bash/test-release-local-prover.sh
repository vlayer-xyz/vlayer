#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/lib/set-missing-git-config.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"

set -ueo pipefail

echo "::group::setting git config"
set_missing_git_config
echo "::endgroup::"

echo "::group::vlayer installation"
curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:$HOME/.config/.vlayer/bin"
vlayerup
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
VLAYER_ENV="dev"
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

for example in $(get_examples); do
    echo "::group::Initializing vlayer template: ${example}"
    VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
    cd ${VLAYER_TEMP_DIR}

    vlayer init --template "${example}"
    forge build
    vlayer test
    echo "::endgroup::"

    echo "Starting docker-compose"
    pushd vlayer
        bun run devnet:up
    popd

    echo "::group::vlayer run prove.ts: ${example}"
    run_prover_script
    echo "::endgroup::"

    pushd vlayer
        bun run devnet:down
    popd
done
