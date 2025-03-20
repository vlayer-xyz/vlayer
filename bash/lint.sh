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

  if [[ -x "$SCRIPT_PATH" ]]; then
    echo "🚀 Running $script..."
    "$SCRIPT_PATH"
    echo "✅ Finished $script."
  else
    echo "❌ Error: $script is missing or not executable."
    exit 1
  fi
done

echo "🎉 All linting scripts completed successfully!"
