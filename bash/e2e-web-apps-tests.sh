#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(get_examples); do
    bash "${VLAYER_HOME}/bash/e2e-web-apps-test.sh" "$example"
done
