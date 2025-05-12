function startup_chain_worker() {
    local db_path="$1" rpc_url="$2" chain_id="$3"

    echo "Starting chain worker with rpc_url=${rpc_url} chain_id=${chain_id}"
    pushd "${VLAYER_HOME}" > /dev/null

    RUST_LOG=${RUST_LOG:-info} ./target/debug/worker \
        --db-path "${db_path}" \
        --rpc-url "${rpc_url}" \
        --chain-id "${chain_id}" \
        --proof-mode "${WORKER_PROOF_ARG}" \
        --confirmations "${CONFIRMATIONS:-1}" \
        --max-head-blocks "${MAX_HEAD_BLOCKS:-10}" \
        --max-back-propagation-blocks "${MAX_BACK_PROPAGATION_BLOCKS:-10}" \
        >> "${LOGS_DIR}/chain_worker_${chain_id}.out" 2>&1 &

    local worker_pid=$!
    echo "Chain worker started with PID ${worker_pid}."
    echo "${worker_pid}" >> "${CHAIN_WORKER_PIDS}"

    popd > /dev/null
}
