#!/usr/bin/env bash

set -euo pipefail

# Usage: configure_rpc.sh <chain_name> <quicknode_api_key>
# Example: configure_rpc.sh sepolia 7c1391dd1fa23de1a735fa4fae2565cc0cc36639

CHAIN_NAME="$1"
QK_KEY="$2"

# Mask the API key in GitHub Actions logs
echo "::add-mask::$QK_KEY"

# Determine the URL fragment and, for Ethereum Sepolia, override the key
case "$CHAIN_NAME" in
  optimismSepolia)
    FRAGMENT="optimism-sepolia"
    ;;
  sepolia)
    FRAGMENT="ethereum-sepolia"
    # override with the known Sepolia API key
    QK_KEY="7c1391dd1fa23de1a735fa4fae2565cc0cc36639"
    ;;
  *)
    # Fallback: convert camelCase -> kebab-case
    FRAGMENT=$(echo "$CHAIN_NAME" \
      | sed -E 's/([a-z0-9])([A-Z])/\1-\L\2/g' \
      | tr '[:upper:]' '[:lower:]')
    ;;
esac

# Export variables for subsequent GitHub Actions steps in one grouped write
{
  echo "QUICKNODE_ENDPOINT=dry-alpha-tab"
  echo "QUICKNODE_API_KEY=$QK_KEY"
  echo "JSON_RPC_URL=https://dry-alpha-tab.${FRAGMENT}.quiknode.pro/$QK_KEY"
} >> "$GITHUB_ENV"
