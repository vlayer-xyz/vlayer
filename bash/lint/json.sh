#! /bin/bash
set -uo pipefail

ERROR_FLAG=$(mktemp -u)
VLAYER_HOME=$(git rev-parse --show-toplevel)

find "$VLAYER_HOME" -type f -name "*.json" ! -path "*/node_modules/*" ! -path "*/target/*" ! -path "*/out/*" ! -path "*/dependencies/*" ! -path "*/.vscode/*" | while read -r file; do
  if ! jq empty "$file" > /dev/null 2>&1; then
    echo "----------------------------------------"
    jq empty "$file"
    echo "Invalid JSON: $file"
    echo "----------------------------------------"
    touch "$ERROR_FLAG"
  else
    echo "Linting passed for $file"
  fi
done

if [[ -f "$ERROR_FLAG" ]]; then
  rm "$ERROR_FLAG"
  echo "Linting failed"
  exit 1
fi
