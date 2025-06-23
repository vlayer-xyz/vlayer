#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

source "$(dirname "${BASH_SOURCE[0]}")/lib/colors.sh"

usage() {
    echo_color YELLOW "Usage: $0 [OPTIONS]"
    echo_color YELLOW "Options:"
    echo_color YELLOW " --help      Display this help message"
    echo_color YELLOW " --nightly   Download the browser extension version from vouch repo (default)"
    echo_color YELLOW " --latest    Download the browser extension version from the latest release to Chrome web store"
}

handle_options() {
    while [ $# -gt 0 ]; do
        case $1 in
            --help)
                usage
                exit 0
                ;;
            --latest)
                version="latest"
                echo_color RED "Downloading the latest version from Chrome web store is not supported yet."
                exit 1
                ;;
            --nightly)
                version="nightly"
                ;;
            *)
                echo_color RED "Invalid option: $1" >&2
                usage
                exit 1
                ;;
        esac
        shift
    done
}

version="nightly"
handle_options "$@"

echo_color BLUE "Reading VOUCH_EXTENSION_DOWNLOAD_PAT from 1Password"
VOUCH_EXTENSION_DOWNLOAD_PAT=$(op read op://Vouch-Engineering/GITHUB_VOUCH_PAT/credential)

echo_color BLUE "Creating directory for browser extension"
cd "${VLAYER_HOME}/packages"
mkdir -p browser-extension
cd browser-extension

echo_color BLUE "Downloading browser extension release from GitHub (${version})"
RELEASE_JSON=$(curl -s -L \
    -H "Accept: application/vnd.github+json" \
    -H "Authorization: Bearer ${VOUCH_EXTENSION_DOWNLOAD_PAT}" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    "https://api.github.com/repos/vlayer-xyz/vouch/releases/tags/browser-extension-${version}")

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
