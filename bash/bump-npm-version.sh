#!/bin/bash
set -ueo pipefail

if [[ -z "${VLAYER_BUILD:-}" ]]; then
  echo "Error: VLAYER_BUILD is not set."
  exit 1
fi

UPDATE_DEPS=false
PEER_DEPENDENCIES=()

for arg in "$@"; do
  case $arg in
    #for now only peer dependencies are needed to be supported
    --update-peer-deps=*) 
      UPDATE_DEPS=true
      PEER_DEPENDENCIES+=("${arg#*=}")
      ;;
    *) 
      echo "Unknown option: $arg"
      exit 1
      ;;
  esac
done

# Update package.json version
jq --arg version "$VLAYER_BUILD" '
  .version = $version
' package.json > package.json.tmp && mv package.json.tmp package.json

echo "Version updated to: $VLAYER_BUILD"

# Optionally update peerDependencies if --update-peer-deps is provided
if $UPDATE_DEPS && [[ ${#PEER_DEPENDENCIES[@]} -gt 0 ]]; then
  for PEER_DEPENDENCY_NAME in "${PEER_DEPENDENCIES[@]}"; do
    jq --arg version "$VLAYER_BUILD" --arg dependency "$PEER_DEPENDENCY_NAME" '
      .peerDependencies[$dependency] = $version
    ' package.json > package.json.tmp && mv package.json.tmp package.json
    echo "Updated package.json with peerDependency $PEER_DEPENDENCY_NAME: $VLAYER_BUILD"
  done
fi

