#!/usr/bin/env bash

set -euo pipefail

if ! command -v shellcheck >/dev/null 2>&1; then
    echo "❌ Error: ShellCheck is not installed."
    echo "Try: brew install shellcheck"
    exit 1
fi

BASH_SCRIPTS_DIR="bash"
find "$BASH_SCRIPTS_DIR" -type f -name "*.sh" -exec shellcheck {} +
