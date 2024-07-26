#!/usr/bin/env bash

set -uexo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

PROVING_MODE=${PROVING_MODE:-dev}

${VLAYER_HOME}/bash/prove-example.sh \
  --mode ${PROVING_MODE} \
  --example simple \
  --caller 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 \
  --data 0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002 

${VLAYER_HOME}/bash/prove-example.sh \
  --mode ${PROVING_MODE} \
  --example simple_travel \
  --caller 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 \
  --data 0xe56a41f9
