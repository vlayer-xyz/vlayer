#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
CURRENT_ELF_ID_PATH="${VLAYER_HOME}/rust/guest_wrapper/artifacts/chain_guest/elf_id"
ELF_ID_HISTORY_PATH="${VLAYER_HOME}/rust/guest_wrapper/artifacts/chain_guest/elf_id_history"

CURRENT_ELF_ID=$(cat "${CURRENT_ELF_ID_PATH}")
PREV_ELF_ID=$(git show --raw HEAD~1:rust/guest_wrapper/artifacts/chain_guest/elf_id)

if [[ "${CURRENT_ELF_ID}" == "${PREV_ELF_ID}" ]]; then
    echo "Chain guest ELF ID unchanged"
    exit 0
fi

grep "${PREV_ELF_ID}" "${ELF_ID_HISTORY_PATH}" > /dev/null && exit 0

echo "Previous ELF ID ${PREV_ELF_ID} not found in history"
exit 1
