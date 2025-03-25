#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/utils.sh"

set_proving_mode
generate_ts_bindings
build_extension

echo "🚀 Running services for example: $EXAMPLE"
DOCKER_COMPOSE_SERVICES="anvil-l1 anvil-l2-op notary-server"

if [[ "${VLAYER_ENV:-dev}" == "dev" ]]; then
  DOCKER_COMPOSE_SERVICES="${DOCKER_COMPOSE_SERVICES} wsproxy-jwt"
else
  DOCKER_COMPOSE_SERVICES="${DOCKER_COMPOSE_SERVICES} wsproxy"
fi

source "${VLAYER_HOME}/bash/run-services.sh"

run_web_tests "$EXAMPLE"

cleanup
