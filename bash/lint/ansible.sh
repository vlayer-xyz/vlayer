#!/usr/bin/env bash

set -euo pipefail

if ! command -v ansible-lint > /dev/null; then
    echo "❌ Error: ansible-lint is not installed."
    echo "Try: brew install ansible-lint"
    exit 1
fi

ANSIBLE_DIR="ansible"
pushd "$ANSIBLE_DIR" > /dev/null
ansible-lint
popd > /dev/null
