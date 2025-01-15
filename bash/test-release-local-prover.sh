#!/usr/bin/env bash

source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib.sh"

set -ueo pipefail

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

VLAYER_ENV="dev"
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

for example in $(get_examples); do
    # We're restarting anvils because some examples rely on a clean chain state.
    echo "Restarting anvils"
    docker compose -f ${VLAYER_HOME}/docker/docker-compose.devnet.yaml restart anvil-l1 anvil-l2-op

    TEMPLATE="${example//_/-}"
    echo "::group::Initializing vlayer template: ${TEMPLATE}"
    VLAYER_TEMP_DIR=$(mktemp -d -t vlayer-test-release-XXXXXX-)
    cd ${VLAYER_TEMP_DIR}

    vlayer init --template "${TEMPLATE}"
    forge build
    vlayer test
    echo '::endgroup::'

    run_prover_script
done
