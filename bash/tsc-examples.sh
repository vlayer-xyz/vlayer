#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

echo "::group::Running bun install for: $VLAYER_HOME/packages"
cd "${VLAYER_HOME}/packages"
bun install --frozen-lockfile
echo '::endgroup::'

echo "::group::Building sdk"
cd "${VLAYER_HOME}/packages/sdk"
bun run build
echo '::endgroup::'


EXAMPLES="simple web_proof"

for example_name in ${EXAMPLES}; do

  example="${VLAYER_HOME}/examples/${example_name}"

  echo ""::group::Running tsc for: ${example}""
  cd "${example}"

  forge soldeer install
  forge build

  echo Generating typescript bidings ...
  ${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null

  cd ${example}/vlayer

  bun install --frozen-lockfile
  bun tsc --noEmit
  echo '::endgroup::'
done
