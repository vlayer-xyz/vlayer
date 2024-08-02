#!/usr/bin/env bash

set -uexo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

### build rust artifacts
( 
  cd "${VLAYER_HOME}/rust" 
  unset RISC0_SKIP_BUILD # ensure RISC0_SKIP_BUILD is not set
  cargo build --release --package guest_wrapper 
)

### copy ImageID to contracts directory 
cp "${VLAYER_HOME}/rust/target/release/assets/ImageID.sol" "${VLAYER_HOME}/contracts/src/"


