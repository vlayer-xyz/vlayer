#!/usr/bin/env bash
set -euo pipefail

RPC_URL=${1:-}
THRESHOLD_GWEI=${2:-10}

if [[ -z "$RPC_URL" ]]; then
  echo "Usage: $0 <RPC_URL>" >&2
  exit 1
fi


response=$(curl -v -X POST "$RPC_URL" \
  -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"eth_gasPrice","params":[],"id":1}')

echo "Raw response: $response"

gas_price_hex=$(echo "$response" | jq -r '.result')

if [[ "$gas_price_hex" == "null" || -z "$gas_price_hex" ]]; then
  echo "Error: Failed to fetch gas price"
  exit 2
fi

gas_price_wei=$((gas_price_hex))
gas_price_gwei=$(awk "BEGIN {printf \"%.4f\", $gas_price_wei/1e9}")

echo "Gas price: $gas_price_gwei gwei"

if awk "BEGIN {exit !($gas_price_gwei <= $THRESHOLD_GWEI)}"; then
  echo "✅ Gas price is low enough → OK"
  exit 0
else
  echo "❌ Gas price is too high → SKIP"
  exit 1
fi
