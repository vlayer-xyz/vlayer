#!/usr/bin/env bash

set -ueo pipefail

# Imports
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib"

# Defaults
PROVING_MODE=${PROVING_MODE:-dev}

generate_ts_bindings
build_sdk

run_examples