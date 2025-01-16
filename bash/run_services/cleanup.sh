source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"

function cleanup() {
    echo "Cleaning up..."

    for service in CHAIN_SERVER VLAYER_SERVER ; do 
        kill_service "${service}"
    done

    while read worker_pid; do
        if ps -p "${worker_pid}" >/dev/null; then
            echo "Killing worker ${worker_pid}"
            kill "${worker_pid}"
        fi
    done < "${CHAIN_WORKER_PIDS}"

    docker compose -f $DOCKER_COMPOSE_FILE down anvil-l1 anvil-l2-op

    echo "Cleanup done. Artifacts saved to: ${VLAYER_TMP_DIR}"
}
