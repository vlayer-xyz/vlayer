#!/usr/bin/env bash

set -euo pipefail

ANSIBLE_DIR="ansible"

if [ ! -d "$ANSIBLE_DIR" ]; then
    echo "Error: '$ANSIBLE_DIR' directory not found!"
    exit 1
fi

echo "::group::Running ansible-lint"
pushd "$ANSIBLE_DIR" > /dev/null
ansible-lint
popd > /dev/null
echo "::endgroup::Ansible lint completed successfully"
