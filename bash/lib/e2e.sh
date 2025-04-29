source "$(dirname "${BASH_SOURCE[0]}")/../lib/io.sh"

function install_chromium() {    
  silent_unless_fails bunx playwright install --with-deps chromium
}

function run_playwright_tests() {
  pushd vlayer
  if grep -q "test-web:${VLAYER_ENV}" package.json; then
    echo "Using test-web:${VLAYER_ENV}"
    export WEB_SERVER_COMMAND="PATH=$PATH:~/.bun/bin bun run web:${VLAYER_ENV}"
    if ! bun run test-web:"${VLAYER_ENV}"; then
      echo "Playwright tests failed"
      return 1
    fi
  else
    echo "Skipping playwright tests as neither test-web:${VLAYER_ENV} nor test-web:${VLAYER_ENV} script exists in package.json"
  fi
  popd
  return 0
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
    SHOULD_DEPLOY_VERIFIER_ROUTER=true bun run prove:"${VLAYER_ENV}"
  popd
  echo "::endgroup::Running prover script"
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
[sol-dependencies.vlayer]
path = "$VLAYER_HOME/contracts/vlayer"
remappings = [["vlayer-0.1.0/", "dependencies/vlayer-0.1.0/src/"]]
[sol-dependencies."@openzeppelin-contracts"]
version = "5.0.1"
remappings = [["openzeppelin-contracts/", "dependencies/@openzeppelin-contracts-5.0.1/"]]
[sol-dependencies.forge-std]
version = "1.9.4"
remappings = [
  ["forge-std/", "dependencies/forge-std-1.9.4/src/"],
  ["forge-std-1.9.4/src/", "dependencies/forge-std-1.9.4/src/"]
]
[sol-dependencies.risc0-ethereum]
version = '2.0.0'
url = "https://github.com/vlayer-xyz/risc0-ethereum/releases/download/v2.0.0-soldeer/contracts.zip"
remappings = [["risc0-ethereum-2.0.0/", "dependencies/risc0-ethereum-2.0.0/"]]
[js-dependencies]
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

function ensure_cli_built() {
  if [[ "${BUILD_CLI}" == "1" ]] ; then
    pushd "${VLAYER_HOME}"
    silent_unless_fails cargo build --bin vlayer
    popd
  fi
}

function run_web_tests() {
  echo "::group::Running playwright tests for ${1} example"

  pushd ${VLAYER_HOME}/examples/${1}

  forge build

  cd vlayer

  bun install --frozen-lockfile
  SHOULD_DEPLOY_VERIFIER_ROUTER=true bun run test-web:"${VLAYER_ENV}"
  
  popd
  echo "::endgroup::Running playwright tests for ${1} example"
}
