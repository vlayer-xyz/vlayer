#!/usr/bin/env bash

set -euo pipefail

ANSIBLE_DIR="ansible"

if ! command -v ansible-lint > /dev/null; then
    echo "❌ Error: ansible-lint is not installed."
    echo "Try: brew install ansible-lint"
    exit 1
fi

pushd "$ANSIBLE_DIR" > /dev/null
ansible-lint
popd > /dev/null
