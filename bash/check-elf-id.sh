#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
CURRENT_ELF_ID_PATH="${VLAYER_HOME}/rust/guest_wrapper/artifacts/chain_guest/elf_id"
ELF_ID_HISTORY_PATH="${VLAYER_HOME}/rust/guest_wrapper/artifacts/chain_guest/elf_id_history"
CHANGELOG_PATH="${VLAYER_HOME}/rust/guest_wrapper/artifacts/chain_guest/CHANGELOG.md"

CURRENT_ELF_ID=$(cat "${CURRENT_ELF_ID_PATH}")
PREV_ELF_ID=$(git show --raw HEAD~1:rust/guest_wrapper/artifacts/chain_guest/elf_id)

if [[ "${CURRENT_ELF_ID}" == "${PREV_ELF_ID}" ]]; then
    echo "Chain guest ELF ID unchanged"
    exit 0
fi

if ! grep -q "${PREV_ELF_ID}" "${ELF_ID_HISTORY_PATH}"; then
  echo "Previous ELF ID ${PREV_ELF_ID} not found in history"
  exit 1
fi

if ! grep "${CURRENT_ELF_ID}" "${CHANGELOG_PATH}" | grep -vq "TODO"; then
  echo "Fill changelog entry for ${CURRENT_ELF_ID}"
  exit 1
fi
