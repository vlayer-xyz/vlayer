#!/usr/bin/env bash

set -uexo pipefail

VLAYER_HOME=$(git rev-parse --show-toplevel)
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"

output_dir="${VLAYER_HOME}/out"
ARCHIVE="${output_dir}/examples.tar"

# Create the output directory if it doesn't exist
mkdir -p "${output_dir}"

if [[ -f "${ARCHIVE}" ]]; then
    rm "${ARCHIVE}"
fi
touch "${ARCHIVE}"

(
    cd "${VLAYER_HOME}/examples"

    for example in $(get_examples); do
        echo "::group::Packing example: ${example}"

        scripts="${example}/vlayer"
        contracts="${example}/src/vlayer"
        contracts_tests="${example}/test/vlayer"
        testdata="${example}/testdata"

        cp "${VLAYER_HOME}/docker/docker-compose.devnet.yaml" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/anvil" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/vdns_server" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/call_server" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/websockify" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/websockify-test-client" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/notary-server" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/notary-config" "${scripts}/"
        cp -a "${VLAYER_HOME}/docker/fixtures" "${scripts}/"

        tar --append --file=$ARCHIVE --strip 1 --exclude-from "${VLAYER_HOME}/examples/.gitignore" --dereference "${contracts}"
        tar --append --file=$ARCHIVE --strip 1 --exclude-from "${VLAYER_HOME}/examples/.gitignore" --dereference "${scripts}"

        if [ -d "${contracts_tests}" ]; then
            tar --append --file=$ARCHIVE --strip 1 --exclude-from "${VLAYER_HOME}/examples/.gitignore" --dereference "${contracts_tests}"
        else
            echo "No tests found for ${example}"
        fi

        if [ -d "${testdata}" ]; then
            tar --append --file=$ARCHIVE --strip 1 --exclude-from "${VLAYER_HOME}/examples/.gitignore" --dereference "${testdata}"
        else
            echo "No testdata found for ${example}"
        fi

        echo "::endgroup::"
    done
)

gzip -f "${ARCHIVE}"
