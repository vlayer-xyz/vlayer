source "$(dirname "${BASH_SOURCE[0]}")/utils.sh"

function build_contracts() {
    echo "::group::Building ${1} contracts"
    pushd "${VLAYER_HOME}/contracts/${1}"
    silent_unless_fails forge soldeer install
    forge clean
    silent_unless_fails forge build
    popd
    echo "::endgroup::Building ${1} contracts"
}

function build_all_contracts() {
  echo "::group::Building contracts"

  mock_imageid
  build_contracts vlayer
  build_contracts fixtures
  generate_ts_bindings

  echo '::endgroup::Building contracts'
}