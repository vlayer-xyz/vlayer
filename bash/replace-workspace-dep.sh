#!/usr/bin/env bash

set -ueo pipefail

path_to_package=${1:-}
dependency=${2:-}

if [[ ! -f "$path_to_package/package.json" ]]; then
  echo "package.json file not found: $path_to_package"
  exit 1
fi

dependency_version=$(jq -r '.version' "$(dirname "$path_to_package")/../packages/$dependency/package.json")

if [[ "$dependency_version" == "null" ]]; then
  echo "Dependency $dependency not found in $path_to_package"
  exit 1
fi

if jq -e --arg dep "$dependency" '.dependencies[$dep] == "workspace:*"' "$path_to_package" > /dev/null; then
  jq --arg dep "$dependency" --arg version "$dependency_version" '
    .dependencies[$dep] = $version
  ' "$path_to_package" > tmp.$$.json && mv tmp.$$.json "$path_to_package"

  echo "Replaced workspace:* with $dependency_version for $dependency in $path_to_package"
else
  echo "Dependency $dependency is not set to workspace:*. Current version is $dependency_version in $path_to_package"
fi
