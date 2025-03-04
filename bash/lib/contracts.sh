source "$(dirname "${BASH_SOURCE[0]}")/utils.sh"

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