set -ueo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"

set_proving_mode

echo Generating typescript bidings ...
${VLAYER_HOME}/bash/build-ts-types.sh >/dev/null
bun install --frozen-lockfile

echo '::group::Build extension'
pushd "$VLAYER_HOME/packages/browser-extension" > /dev/null
bun run build
popd > /dev/null
echo '::endgroup::'

install_chromium

for example in $(get_examples); do
  echo Running services...
  source ${VLAYER_HOME}/bash/run-services.sh
  echo "::group::Running tests of: ${example}"
  cd "$VLAYER_HOME/examples/$example"
  forge build
  cd vlayer
  echo "Running tests for ${example}"
  pushd "$VLAYER_HOME/examples/$example/" > /dev/null
    run_playwright_tests 
  popd > /dev/null
done
