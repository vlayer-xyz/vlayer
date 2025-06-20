#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

source "$(dirname "${BASH_SOURCE[0]}")/lib/colors.sh"

echo_color BLUE "Reading VOUCH_EXTENSION_DOWNLOAD_PAT from 1Password"
VOUCH_EXTENSION_DOWNLOAD_PAT=$(op read op://Vouch-Engineering/GITHUB_VOUCH_PAT/credential)

echo_color BLUE "Creating directory for browser extension"
cd "${VLAYER_HOME}/packages"
mkdir -p browser-extension
cd browser-extension

echo_color BLUE "Downloading latest browser extension release from GitHub"
RELEASE_JSON=$(curl -s -L \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer ${VOUCH_EXTENSION_DOWNLOAD_PAT}" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "https://api.github.com/repos/vlayer-xyz/vouch/releases/tags/browser-extension-nightly")

ASSET_URL=$(echo "$RELEASE_JSON" | jq -r '.assets[] | select(.name == "browser-extension.tar.gz") | .url')
echo_color YELLOW "Asset URL: $ASSET_URL"

curl -L \
    -H "Accept: application/octet-stream" \
    -H "Authorization: Bearer ${VOUCH_EXTENSION_DOWNLOAD_PAT}" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "$ASSET_URL" \
    --output browser-extension.tar.gz

echo_color BLUE "Extracting browser extension files to dist directory"
mkdir -p dist
tar -xzf browser-extension.tar.gz -C dist --strip-components=4
rm browser-extension.tar.gz

echo_color GREEN "Browser extension downloaded successfully to ${VLAYER_HOME}/packages/browser-extension/dist"