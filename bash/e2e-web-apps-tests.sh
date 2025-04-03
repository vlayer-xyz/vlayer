#!/usr/bin/env bash

set -ueo pipefail

source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

VLAYER_HOME=$(git rev-parse --show-toplevel)

for example in $(get_examples); do
    if [[ "$example" == "simple-email-proof" ]]; then
        continue
    fi
    EXAMPLE="$example" bash "${VLAYER_HOME}/bash/e2e-web-apps-test.sh"
done
