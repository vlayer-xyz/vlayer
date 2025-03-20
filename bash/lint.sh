#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

SCRIPTS=(
  "ansible"
  "bash"
  "json"
  "rust"
  "solidity"
  "ts"
  "workflows"
)

for script in "${SCRIPTS[@]}"; do
  SCRIPT_PATH="$SCRIPT_DIR/lint/$script.sh"
  "$SCRIPT_PATH"
done

echo "🎉 All linting scripts completed successfully!"
