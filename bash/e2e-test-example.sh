#!/usr/bin/env bash

set -ueo pipefail

# Imports
VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/e2e/lib.sh"

# Defaults
set_proving_mode
VLAYER_ENV=${VLAYER_ENV:-dev}

export EXAMPLE_NAME=$EXAMPLE
echo Running services...
source ${VLAYER_HOME}/bash/run-services.sh

pushd $(mktemp -d)

cat <<EOF > config.toml
template = "$EXAMPLE"

[contracts.vlayer]
path = "$VLAYER_HOME/contracts/vlayer"
remappings = [["vlayer-0.1.0/", "dependencies/vlayer-0.1.0/src/"]]

[contracts."@openzeppelin-contracts"]
version = "5.0.1"
remappings = [["openzeppelin-contracts/", "dependencies/@openzeppelin-contracts-5.0.1/"]]

[contracts.forge-std]
version = "1.9.4"
remappings = [
  ["forge-std/", "dependencies/forge-std-1.9.4/src/"],
  ["forge-std-1.9.4/src/", "dependencies/forge-std-1.9.4/src/"]
]

[contracts.risc0-ethereum]
version = '1.2.0'
url = "https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v1.2.0-soldeer/contracts.zip"
remappings = [["risc0-ethereum-1.2.0/", "dependencies/risc0-ethereum-1.2.0/"]]

[npm]
"@vlayer/sdk" = { path = "$VLAYER_HOME/packages/sdk" }
"@vlayer/react" = { path = "$VLAYER_HOME/packages/sdk-hooks" }
EOF

$VLAYER_HOME/target/debug/vlayer init $EXAMPLE --templates-dir $VLAYER_HOME/examples --config-file config.toml

pushd $EXAMPLE
cat remappings.txt
forge build

pushd vlayer
cat package.json
bun install --no-cache
bun run prove:dev
# run_prover_script
popd

popd

popd

cleanup
rm -rf "${VLAYER_TMP_DIR}/chain_db"
