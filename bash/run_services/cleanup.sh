source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

function cleanup() {
    echo "::group::Cleaning up"

    for service in CHAIN_SERVER VLAYER_SERVER DNS_SERVER ; do
        kill_service "${service}"
    done

    while read worker_pid; do
        if ps -p "${worker_pid}" >/dev/null; then
            echo "Killing worker ${worker_pid}"
            kill "${worker_pid}"
        fi
    done < "${CHAIN_WORKER_PIDS}"

    if [[ $VLAYER_ENV == "dev" ]]; then
        docker compose ${DOCKER_COMPOSE_FILES[@]} down
    fi

    echo "Artifacts saved to: ${VLAYER_TMP_DIR}"

    echo "::endgroup::Cleaning up"
}
