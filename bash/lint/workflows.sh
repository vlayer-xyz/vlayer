#!/usr/bin/env bash

set -euo pipefail

WORKFLOWS_DIR=".github/workflows"

if ! command -v prettier >/dev/null; then
    echo "❌ Error: Prettier is not installed."
    echo "Try: npm install --global prettier"
    exit 1
fi

if ! command -v actionlint >/dev/null; then
    echo "❌ Error: actionlint is not installed. Try:"
    echo "   curl -fsSL \"https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash\" | bash"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    exit 1
fi

prettier --check "$WORKFLOWS_DIR/**/*.yaml"
actionlint
