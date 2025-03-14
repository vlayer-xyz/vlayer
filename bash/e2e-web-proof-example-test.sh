#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/utils.sh"

set_proving_mode

generate_ts_bindings

echo "::group::Running services"
DOCKER_COMPOSE_SERVICES="anvil-l1 anvil-l2-op wsproxy notary-server"
source ${VLAYER_HOME}/bash/run-services.sh
echo "::endgroup::Running services"

build_extension

run_web_tests simple-web-proof

echo "::group::Cleanup"
cleanup
echo "::endgroup::Cleanup"
