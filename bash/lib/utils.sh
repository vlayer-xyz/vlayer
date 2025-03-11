source "$(dirname "${BASH_SOURCE[0]}")/io.sh"

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
