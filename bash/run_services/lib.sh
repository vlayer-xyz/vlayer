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

function concat() {
    local IFS="$1"
    shift
    echo "$*"
}

function startup_vlayer() {
    local proof_arg=$1
    shift # shift input params, since the second (and last) arg is an array of external_urls 
    local rpc_urls=("$@")

    local chain_client_url=
    local jwt_pub_key=
    local jwt_algorithm=

    echo "Starting vlayer REST server"
    pushd "${VLAYER_HOME}"

    rpc_urls+=(
        "31337:http://localhost:8545" # L1
        "31338:http://localhost:8546" # L2 OP
    )
    rpc_urls_concat=$(concat " " ${rpc_urls[@]+"${rpc_urls[@]}"})

    if [[ "${JWT_AUTH}" == "on" ]]; then
        jwt_pub_key="./docker/fixtures/jwt-authority.key.pub" # JWT public key
        jwt_algorithm="rs256" # JWT signing algorithm
    fi

    if [[ -n "${EXTERNAL_CHAIN_SERVICE_URL:-}" ]]; then
        chain_client_url="${EXTERNAL_CHAIN_SERVICE_URL}"
    elif [[ ${#CHAIN_WORKER_ARGS[@]} -gt 0 ]]; then
        chain_client_url="http://localhost:3001"
    fi

    RUST_LOG=info \
    BONSAI_API_URL="${BONSAI_API_URL}" \
    BONSAI_API_KEY="${BONSAI_API_KEY}" \
    VLAYER_PROOF_MODE="${proof_arg}" \
    VLAYER_RPC_URLS="${rpc_urls_concat}" \
    VLAYER_CHAIN_CLIENT__URL="${chain_client_url}" \
    VLAYER_AUTH__JWT__PUBLIC_KEY="${jwt_pub_key}" \
    VLAYER_AUTH__JWT__ALGORITHM="${jwt_algorithm}" \
    ./target/debug/call_server >>"${LOGS_DIR}/vlayer_serve.out" &

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
