source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"
CHAIN_NAME=${CHAIN_NAME:-anvil}

declare -A NETWORK_IDS=(
    ["ethereum-mainnet"]=1
    ["ethereum-sepolia"]=11155111
    ["optimism"]=10
    ["optimism-sepolia"]=11155420
    ["base-mainnet"]=8453
    ["base-sepolia"]=84532
)

ANVIL_RPC_URL="http://localhost:8545"
ANVIL_CHAIN_ID="31337"

get_quiknode_url() {
    local network="$1"
    local id="${NETWORK_IDS[$network]}"
    echo "https://${QUICKNODE_ENDPOINT}.${network}.quiknode.pro/${QUICKNODE_API_KEY}"
}

function set_proof_mode() {
    if [[ "${PROVING_MODE}" == "dev" ]]; then
        SERVER_PROOF_ARG="fake"
        WORKER_PROOF_ARG="fake"
    elif [[ "${PROVING_MODE}" == "prod" ]]; then
        SERVER_PROOF_ARG="groth16"
        WORKER_PROOF_ARG="succinct"

        if [[ -z "$BONSAI_API_URL" || -z "$BONSAI_API_KEY" ]]; then
            echo "Error: BONSAI_API_URL and BONSAI_API_KEY must be set in prod mode."
            usage
        fi
    fi
}

function set_external_rpc_urls() {
    if [[ "$CHAIN_NAME" == "anvil" ]]; then
        return 0
    fi
    if [[ -z "$QUICKNODE_API_KEY" || -z "$QUICKNODE_ENDPOINT" ]]; then
        echo "Error: QUICKNODE_API_KEY and QUICKNODE_ENDPOINT must be set in prod mode."
        usage
    fi
    EXTERNAL_RPC_URLS=()
    for network in "${!NETWORK_IDS[@]}"; do
        EXTERNAL_RPC_URLS+=(
            "--rpc-url" "${NETWORK_IDS[$network]}:$(get_quiknode_url "$network")"
        )
    done
}

function set_chain_worker_args() {
    CONFIRMATIONS=${CONFIRMATIONS:-1}
    CHAIN_WORKER_ARGS=()

    if [[ "$CHAIN_NAME" == "anvil" && "${EXAMPLE_NAME:-}" == "simple-time-travel" ]]; then
        CHAIN_WORKER_ARGS=("${ANVIL_RPC_URL}" "${ANVIL_CHAIN_ID}")
        return 0
    fi

    for network in "${!NETWORK_IDS[@]}"; do
        CHAIN_WORKER_ARGS+=("$(get_quiknode_url "$network")" "${NETWORK_IDS[$network]}")
    done
}
