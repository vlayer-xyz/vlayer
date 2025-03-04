source "$(dirname "${BASH_SOURCE[0]}")/io.sh"
source "$(dirname "${BASH_SOURCE[0]}")/contracts.sh"

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

function build_react_sdk_with_deps() {
  echo "::group::Building react sdk with dependencies"

  bun install --frozen-lockfile

  build_contracts

  build_sdk
  build_sdk_hooks

  echo "::endgroup::Building react sdk with dependencies"
}
