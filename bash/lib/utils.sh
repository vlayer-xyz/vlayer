source "$(dirname "${BASH_SOURCE[0]}")/io.sh"

function generate_ts_bindings() {
  echo "::group::Generating typescript bidings"
  silent_unless_fails ${VLAYER_HOME}/bash/build-ts-types.sh
  echo "::endgroup::Generating typescript bidings"
}
