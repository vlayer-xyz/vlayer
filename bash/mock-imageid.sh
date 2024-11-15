#!/usr/bin/env bash

set -uexo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
IMAGE_ID_FILE="${VLAYER_HOME}/contracts/vlayer/src/ImageID.sol"

if [[ -L "${IMAGE_ID_FILE}" ]] ; then # check whether ImageID is a symlink
    rm "${IMAGE_ID_FILE}"
fi

cat <<EOF >"${IMAGE_ID_FILE}"
pragma solidity ^0.8.20;

library ImageID {
    bytes32 public constant RISC0_CALL_GUEST_ID =
        bytes32(0x0000000000000000000000000000000000000000000000000000000000000000);
}
EOF
