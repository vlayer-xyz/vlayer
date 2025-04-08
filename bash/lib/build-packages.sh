source "$(dirname "${BASH_SOURCE[0]}")/io.sh"
source "$(dirname "${BASH_SOURCE[0]}")/build-contracts.sh"


function build_package () {
  echo "::group::Building ${1}"
  pushd "${VLAYER_HOME}/packages/${1}"
  silent_unless_fails bun install --frozen-lockfile
  silent_unless_fails bun run build
  popd
  echo "::endgroup::Building ${1}"
}

function build_sdk() {
  build_package sdk
}

function build_sdk_hooks() {
  build_package sdk-hooks
}

function build_react_sdk_with_deps() {
  echo "::group::Building react sdk with dependencies"

  bun install --frozen-lockfile

  build_sdk
  build_sdk_hooks

  echo "::endgroup::Building react sdk with dependencies"
}

function build_extension() {
  build_package browser-extension
}
