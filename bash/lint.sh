#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

SCRIPTS=(
  "bash"
  "json"
  "workflows"
  "rust"
  "solidity"
  "ts"
  "ansible"
)

for script in "${SCRIPTS[@]}"; do
  SCRIPT_PATH="$SCRIPT_DIR/lint/$script.sh"
  "$SCRIPT_PATH"
done

echo "🎉 All linting scripts completed successfully!"
