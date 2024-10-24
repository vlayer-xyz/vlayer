#!/usr/bin/env bash

set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)

cd "${VLAYER_HOME}/packages"
echo "Running bun install for: $VLAYER_HOME/packages"
bun install --frozen-lockfile

EXAMPLES="simple web_proof"

for example_name in ${EXAMPLES}; do

  example="${VLAYER_HOME}/examples/${example_name}"

  echo "Running tsc for: ${example}"
  cd "${example}"

  forge soldeer install
  forge build

  echo Generating typescript bidings ...
  ${VLAYER_HOME}/bash/build_ts_types.sh >/dev/null

  cd ${example}/vlayer

  bun install --frozen-lockfile
  bun tsc --noEmit
done
