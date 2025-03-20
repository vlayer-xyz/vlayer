#!/usr/bin/env bash

set -euo pipefail

BASH_SCRIPTS_DIR="bash"

if ! command -v shellcheck >/dev/null 2>&1; then
    echo "❌ Error: ShellCheck is not installed."
    echo "Try: brew install shellcheck"
    exit 1
fi

find "$BASH_SCRIPTS_DIR" -type f -name "*.sh" -exec shellcheck {} +
