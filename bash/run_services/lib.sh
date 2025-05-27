source "$(dirname "${BASH_SOURCE[0]}")/../lib/io.sh"
source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/chain_worker.sh"

function setup_tmp_dir() {
    if [[ -z "${VLAYER_TMP_DIR:-}" ]] ; then
        VLAYER_TMP_DIR=$(mktemp -d -t vlayer-$(basename $0).XXXXX)
    else
        VLAYER_TMP_DIR=$(realpath "${VLAYER_TMP_DIR}")
    fi
}

function startup_vdns_server() {
    echo "Starting VDNS server"
    pushd "${VLAYER_HOME}"

    local args=()

    if [[ "${JWT_AUTH}" == "on" ]]; then
        args+=("--jwt-public-key" "./docker/fixtures/jwt-authority.key.pub") # JWT public key
    fi

    RUST_LOG=info \
    ./target/debug/dns_server \
        ${args[@]+"${args[@]}"} \
        >>"${LOGS_DIR}/dns_server.out" &

    DNS_SERVER=$!
    
    echo "DNS server started with PID ${DNS_SERVER}."
    wait_for_port_and_pid 3002 "${DNS_SERVER}" 30m "dns server"

    popd
}

function startup_chain_server() {
    local db_path="$1"

    echo "Starting chain server"
    pushd "${VLAYER_HOME}"

    RUST_LOG=info \
    ./target/debug/chain_server \
        --db-path "${db_path}" \
        >>"${LOGS_DIR}/chain_server.out" &

    CHAIN_SERVER=$!

    echo "Chain server started with PID ${CHAIN_SERVER}."
    wait_for_port_and_pid 3001 "${CHAIN_SERVER}" 30m "chain server"

    popd
}

function startup_chain_services() {
    if [[ -z "$@" ]] ; then
        return 0
    fi

    local db_path="${VLAYER_TMP_DIR}/chain_db"

    for args in "$@"; do 
        startup_chain_worker ${db_path} $args &
    done

    startup_chain_server ${db_path}
}

function startup_vlayer() {
    local proof_arg=$1
    shift # shift input params, since the second (and last) arg is an array of external_urls 
    local external_urls=("$@")

    echo "Starting vlayer REST server"
    pushd "${VLAYER_HOME}"

    local args=(
        "--proof" "${proof_arg}"
        "--rpc-url" "31337:http://localhost:8545" # L1
        "--rpc-url" "31338:http://localhost:8546" # L2 OP
    )

    if [[ "${JWT_AUTH}" == "on" ]]; then
        args+=("--jwt-public-key" "./docker/fixtures/jwt-authority.key.pub") # JWT public key
    fi

    if [[ -n "${EXTERNAL_CHAIN_SERVICE_URL:-}" ]]; then
        args+=("--chain-proof-url" "${EXTERNAL_CHAIN_SERVICE_URL}")
    elif [[ ${#CHAIN_WORKER_ARGS[@]} -gt 0 ]]; then
        args+=("--chain-proof-url" "http://localhost:3001")
    fi

    RUST_LOG=info \
    BONSAI_API_URL="${BONSAI_API_URL}" \
    BONSAI_API_KEY="${BONSAI_API_KEY}" \
    ./target/debug/call_server \
        ${args[@]} \
        ${external_urls[@]+"${external_urls[@]}"} \
        >>"${LOGS_DIR}/vlayer_serve.out" &

    VLAYER_SERVER=$!

    echo "vlayer server started with PID ${VLAYER_SERVER}."
    wait_for_port_and_pid 3000 ${VLAYER_SERVER} 30m "vlayer server"

    popd
}

function ensure_services_built() {
    if [[ "${BUILD_SERVICES}" == "1" ]] ; then
        pushd "${VLAYER_HOME}"
        silent_unless_fails cargo build --bin call_server --bin chain_server --bin worker --bin dns_server
        popd
    fi
}
