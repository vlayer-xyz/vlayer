#!/usr/bin/env bash

set -euo pipefail

WORKFLOWS_DIR=".github/workflows"

check_dependencies() {
    if ! command -v prettier > /dev/null; then
        echo "❌ Error: Prettier is not installed. Please install it manually:"
        echo "   npm install --global prettier"
        exit 1
    fi

    if ! command -v actionlint > /dev/null; then
        echo "❌ Error: actionlint is not installed. Please install it manually:"
        echo "   curl -fsSL \"https://raw.githubusercontent.com/rhysd/actionlint/main/scripts/download-actionlint.bash\" | bash"
        echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
        exit 1
    fi
}

check_dependencies
prettier --check "$WORKFLOWS_DIR/**/*.yaml"
actionlint
