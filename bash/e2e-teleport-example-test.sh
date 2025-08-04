#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/utils.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"

set_proving_mode

generate_ts_bindings

if [[ -z "${WEB_SERVER_URL:-}" ]]; then
  echo '::group::Running services'
  source ${VLAYER_HOME}/bash/run-services.sh
  echo '::endgroup::'
else
  echo "Using remote web server at: ${WEB_SERVER_URL}"
  echo "Skipping local service setup, mocking imageid"
fi

run_web_tests simple-teleport
