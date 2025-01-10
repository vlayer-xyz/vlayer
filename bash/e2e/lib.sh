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
  pushd vlayer
      silent_unless_fails bun install --frozen-lockfile
      bun run prove:"${VLAYER_ENV}"
  popd
}

function build_contracts() {
  forge soldeer install
  forge clean
  forge build
}
