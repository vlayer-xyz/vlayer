function startup_chain_worker() {
    local db_path="$1" rpc_url="$2" chain_id="$3" first_block="$4" last_block="$5"
    local start_block="$last_block"

    echo "Starting chain worker with rpc_url=${rpc_url} chain_id=${chain_id} start_block=${start_block}"
    pushd "${VLAYER_HOME}" > /dev/null

    RUST_LOG=${RUST_LOG:-info} ./target/debug/worker \
        --db-path "${db_path}" \
        --rpc-url "${rpc_url}" \
        --chain-id "${chain_id}" \
        --proof-mode "${WORKER_PROOF_ARG}" \
        --confirmations "${CONFIRMATIONS:-1}" \
        --start-block "${start_block}" \
        --max-head-blocks "${MAX_HEAD_BLOCKS:-10}" \
        --max-back-propagation-blocks "${MAX_BACK_PROPAGATION_BLOCKS:-10}" \
        >> "${LOGS_DIR}/chain_worker_${chain_id}.out" 2>&1 &

    local worker_pid=$!
    echo "Chain worker started with PID ${worker_pid}."
    echo "${worker_pid}" >> "${CHAIN_WORKER_PIDS}"

    popd > /dev/null
}

function wait_for_chain_worker_sync() {
    local rpc_url="$1" chain_id="$2" first_block="$3" last_block="$4"
    local rpc_url="http://127.0.0.1:3001"
    local max_retries=20
    local sleep_seconds=10
    local retry_count=5
    local retry_delay=0
    local retry_max_time=30

    echo "Waiting for chain worker sync... chain_id=${chain_id} first_block=${first_block} last_block=${last_block}"

    for i in $(seq 1 $max_retries); do
        local reply result first_block_synced last_block_synced
        reply=$(curl -s -X POST "${rpc_url}" \
            --retry-connrefused --retry "$retry_count" --retry-delay "$retry_delay" --retry-max-time "$retry_max_time" \
            -H "Content-Type: application/json" \
            --data '{"jsonrpc": "2.0", "id": 0, "method": "v_getSyncStatus", "params": ['"${chain_id}"']}')
        result=$(echo "${reply}" | jq ".result")

        if [[ "${result}" != "null" ]]; then
            first_block_synced=$( [[ "${first_block}" == "latest" ]] || echo "${result}" | jq "(.first_block <= ${first_block})" )
            last_block_synced=$( [[ "${last_block}" == "latest" ]] || echo "${result}" | jq "(.last_block >= ${last_block})" )

            if [[ "${first_block_synced}" == "true" && "${last_block_synced}" == "true" ]]; then
                echo "Chain ${chain_id} worker synced ${result}"
                return
            fi
        fi

        echo "Syncing ... ${result}"
        sleep "$sleep_seconds"
    done

    echo "Failed to sync chain ${chain_id} worker" >&2
    exit 1
}

