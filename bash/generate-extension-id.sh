#!/bin/bash

# Set strict mode
set -ueo pipefail

generate_sha256_hash() {
  echo -n "$1" | openssl dgst -sha256 -binary
}

to_hex() {
  echo "$1" | xxd -p | tr -d '\n'
}

convert_hex_to_id_alphabet() {
  echo "$1" | tr 0-9a-f a-p
}

generate_extension_id_for_path() {
  path_bytes=$(echo -n "$1")
  sha256_hash=$(generate_sha256_hash "$path_bytes" | head -c16)
  hex_encoded=$(to_hex "$sha256_hash")
  hex_encoded=${hex_encoded:0:32}
  convert_hex_to_id_alphabet "$hex_encoded"
}

VLAYER_HOME=$(git rev-parse --show-toplevel)
EXTENSION_PATH="${VLAYER_HOME}/packages/browser-extension/dist"
generate_extension_id_for_path "$EXTENSION_PATH"
