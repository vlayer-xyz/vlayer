#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/utils.sh"

set_proving_mode

generate_ts_bindings

if [[ -z "${WEB_SERVER_URL:-}" ]]; then
  echo "Running with local services"
  ./bash/mock-imageid.sh
else
  echo "Using remote web server at: ${WEB_SERVER_URL}"
  echo "Skipping local service setup"
fi

run_web_tests simple-web-proof
