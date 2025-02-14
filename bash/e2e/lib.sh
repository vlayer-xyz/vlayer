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
  forge build
}

function run_playwright_tests() {
  pushd vlayer
    silent_unless_fails bunx playwright install --with-deps chromium
    WEB_SERVER_COMMAND="PATH=$PATH:~/.bun/bin bun run web:${VLAYER_ENV}" bun run test:"${VLAYER_ENV}"
  popd
}
