source "$(dirname "${BASH_SOURCE[0]}")/../lib/io.sh"

function mock_imageid() {
  echo "::group::Mock ImageId"
  silent_unless_fails ${VLAYER_HOME}/bash/mock-imageid.sh
  echo '::endgroup::Mock ImageId'
}

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

function build_sdk_hooks() {
  echo "::group::Building sdk-hooks"
  pushd "${VLAYER_HOME}/packages/sdk-hooks"
    silent_unless_fails bun install --frozen-lockfile
    silent_unless_fails bun run build
  popd
  echo '::endgroup::Building sdk-hooks'
}

function build_vlayer_contracts() {
  echo "::group::Building vlayer contracts"
  pushd "${VLAYER_HOME}/contracts/vlayer"
  silent_unless_fails forge soldeer install
  forge clean
  silent_unless_fails forge build
  popd
  echo '::endgroup::Building vlayer contracts'
}

function build_fixtures_contracts() {
  echo "::group::Building fixtures contracts"
  pushd "${VLAYER_HOME}/contracts/fixtures"
  silent_unless_fails forge soldeer install
  forge clean
  silent_unless_fails forge build
  popd
  echo '::endgroup::Building fixtures contracts'
}

function build_contracts() {
  echo "::group::Building contracts"

  mock_imageid
  build_vlayer_contracts
  build_fixtures_contracts
  generate_ts_bindings

  echo '::endgroup::Building contracts'
}

function build_all_for_ts() {
  echo "::group::Building all for typescript"

  bun install --frozen-lockfile

  build_contracts

  build_sdk
  build_sdk_hooks

  echo '::endgroup::Building all for typescript'
}