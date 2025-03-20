#! /bin/bash
set -uo pipefail

HAS_ERROR=false
VLAYER_HOME=$(git rev-parse --show-toplevel)

find "$VLAYER_HOME" -type f -name "*.json" ! -path "*/node_modules/*" ! -path "*/target/*" ! -path "*/out/*" ! -path "*/dependencies/*" ! -path "*/.vscode/*" | while read -r file; do
  if ! jq empty "$file" > /dev/null 2>&1; then
    echo "----------------------------------------"
    jq empty "$file"
    echo "Invalid JSON: $file" 
    echo "----------------------------------------"
    HAS_ERROR=true
  fi
done

if $HAS_ERROR; then
  exit 1
fi