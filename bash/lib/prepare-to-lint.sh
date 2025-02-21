function prepare-to-lint() {
    bun install --frozen-lockfile

    echo "::group::Building contracts"

    ./bash/mock-imageid.sh

    echo Run Forge build vlayer
    pushd ${VLAYER_HOME}/contracts/vlayer
    forge soldeer install
    forge clean
    forge build
    popd

    echo Run Forge build fixtures
    pushd ${VLAYER_HOME}/contracts/fixtures
    forge soldeer install
    forge clean
    forge build
    popd

    ./bash/build-ts-types.sh

    echo '::endgroup::'

    echo "::group::Building sdk"
    cd "${VLAYER_HOME}/packages/sdk"
    bun install --frozen-lockfile
    bun run build
    echo '::endgroup::'

    echo "::group::Building sdk-hooks"
    cd "${VLAYER_HOME}/packages/sdk-hooks"
    bun install --frozen-lockfile
    bun run build
    echo '::endgroup::'
}