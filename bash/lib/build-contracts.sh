source "$(dirname "${BASH_SOURCE[0]}")/utils.sh"
source "$(dirname "${BASH_SOURCE[0]}")/examples.sh"

function build_contracts_in() {
    echo "::group::Building ${1} contracts"
    pushd "${VLAYER_HOME}/contracts/${1}"
    silent_unless_fails forge soldeer install
    forge clean
    silent_unless_fails forge build
    popd
    echo "::endgroup::Building ${1} contracts"
}

function build_example_contracts() {
    echo "::group::Building ${1} example contracts"
    pushd "${VLAYER_HOME}/examples/${1}"
    silent_unless_fails forge soldeer install
    forge clean
    silent_unless_fails forge build
    popd
    echo "::endgroup::Building ${1} example contracts"
}

function build_core_contracts() {
  echo "::group::Building contracts"

  build_contracts_in vlayer
  build_contracts_in fixtures
  generate_ts_bindings

  echo "::endgroup::Building contracts"
}

function build_contracts() {
  build_contracts_in vlayer
  build_contracts_in fixtures
  for example in $(get_examples); do
    build_example_contracts $example
  done
  generate_ts_bindings
}
