#! /bin/bash
set -uo pipefail

HAS_ERROR=false
VLAYER_HOME=$(git rev-parse --show-toplevel)

shopt -s globstar

for file in "${VLAYER_HOME}/"**; do
  if [[ "$file" == *"/node_modules"* ]]; then
    continue
  fi
  if [[ -f "$file" && "$file" == *.json ]]; then
    if ! jq empty "$file" > /dev/null 2>&1; then
      echo "----------------------------------------"
      jq empty "$file"
      echo "Invalid JSON: $file" 
      echo "----------------------------------------"
      HAS_ERROR=true
    fi
  fi
done

if $HAS_ERROR; then
  exit 1
fi