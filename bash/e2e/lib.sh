source "$(dirname "${BASH_SOURCE[0]}")/../lib/io.sh"

function generate_ts_bindings() {
  echo "::group::Generating typescript bidings"
  silent_unless_fails ${VLAYER_HOME}/bash/build-ts-types.sh
  echo '::endgroup::Generating typescript bidings'
}

function build_sdk() {
  echo "::group::Building sdk"
  pushd "${VLAYER_HOME}/packages/sdk"
    silent_unless_fails bun install --frozen-lockfile
    silent_unless_fails bun run build
  popd
  echo '::endgroup::Building sdk'
}

function run_prover_script() {
  echo "::group::Running prover script"
  # Sadly, bun's manifest caching is so unstable, it causes random `bun install` freezes.
  # To circumvent that for the time being, we disable all caching.
  # https://github.com/oven-sh/bun/issues/5831
  if [[ -n ${BUN_NO_FROZEN_LOCKFILE:-} ]]; then
    local args="--no-cache"
  else
    local args="--frozen-lockfile --no-cache"
  fi
  pushd vlayer
    silent_unless_fails bun install "${args}"
    bun run prove:"${VLAYER_ENV}"
  popd
  echo "::endgroup::Running prover script"
}

function build_vlayer_contracts() {
  pushd "${VLAYER_HOME}/contracts/vlayer"
  build_contracts
  popd
}

function build_contracts() {
  echo "::group::Building contracts"
  forge build
  echo "::endgroup::Building contracts"
}

function run_playwright_tests() {
  pushd vlayer
    silent_unless_fails bunx playwright install --with-deps chromium
    WEB_SERVER_COMMAND="PATH=$PATH:~/.bun/bin bun run web:${VLAYER_ENV}" bun run test:"${VLAYER_ENV}"
  popd
}

function generate_vlayer_init_config() {
  echo "::group::Generating vlayer init config"

  if [[ -z "${EXAMPLE:-}" ]] ; then
    echo "EXAMPLE is unset"
    exit 1
  fi

  if [[ -z "${VLAYER_HOME:-}" ]] ; then
    echo "VLAYER_HOME is unset"
    exit 1
  fi

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

  cat config.toml
  
  echo "::endgroup::Generating vlayer init config"
}

function init_template() {
  if [[ -z "${EXAMPLE:-}" ]] ; then
    echo "EXAMPLE is unset"
    exit 1
  fi

  if [[ -z "${VLAYER_HOME:-}" ]] ; then
    echo "VLAYER_HOME is unset"
    exit 1
  fi

  echo "::group::Initializing from template $EXAMPLE"

  $VLAYER_HOME/target/debug/vlayer init $EXAMPLE --templates-dir $VLAYER_HOME/examples --config-file config.toml

  echo "::endgroup::Initializing from template $EXAMPLE"
}
