#!/usr/bin/env bash

set -uexo pipefail

PROVING_MODE=${PROVING_MODE:-dev}

./bash/prove-example.sh \
  --mode ${PROVING_MODE} \
  --example simple \
  --caller 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 \
  --data 0xcad0899b00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002 

./bash/prove-example.sh \
  --mode ${PROVING_MODE} \
  --example simple_travel \
  --caller 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266 \
  --data 0xe56a41f9
