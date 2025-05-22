source "$(dirname "${BASH_SOURCE[0]}")/../common.sh"
CHAIN_NAME=${CHAIN_NAME:-anvil}

# Set the SERVER_PROOF_MODE variable based on the mode
function set_proof_mode() {
    if [[ "${PROVING_MODE}" == "dev" ]]; then
        SERVER_PROOF_ARG="fake"
        WORKER_PROOF_ARG="fake"
    elif [[ "${PROVING_MODE}" == "prod" ]]; then
        SERVER_PROOF_ARG="groth16"
        WORKER_PROOF_ARG="succinct"

        # Check that BONSAI_API_URL and BONSAI_API_KEY are not empty in prod mode
        if [[ -z "$BONSAI_API_URL" || -z "$BONSAI_API_KEY" ]]; then
            echo "Error: BONSAI_API_URL and BONSAI_API_KEY must be set in prod mode."
            usage
        fi
    fi
}

function set_external_rpc_urls() {
    if [[ "${CHAIN_NAME}" != "anvil" ]]; then
        # Check that QUICKNODE_API_KEY and QUICKNODE_ENDPOINT are not empty
        if [[ -z "$QUICKNODE_API_KEY" || -z "$QUICKNODE_ENDPOINT" ]]; then
            echo "Error: QUICKNODE_API_KEY and QUICKNODE_ENDPOINT must be set in prod mode."
            usage
        fi
        EXTERNAL_RPC_URLS=(
            "--rpc-url" "1:https://${QUICKNODE_ENDPOINT}.quiknode.pro/${QUICKNODE_API_KEY}"
            "--rpc-url" "11155111:https://${QUICKNODE_ENDPOINT}.ethereum-sepolia.quiknode.pro/${QUICKNODE_API_KEY}"
            "--rpc-url" "10:https://${QUICKNODE_ENDPOINT}.optimism.quiknode.pro/${QUICKNODE_API_KEY}"
            "--rpc-url" "11155420:https://${QUICKNODE_ENDPOINT}.optimism-sepolia.quiknode.pro/${QUICKNODE_API_KEY}"
            "--rpc-url" "8453:https://${QUICKNODE_ENDPOINT}.base-mainnet.quiknode.pro/${QUICKNODE_API_KEY}"
            "--rpc-url" "84532:https://${QUICKNODE_ENDPOINT}.base-sepolia.quiknode.pro/${QUICKNODE_API_KEY}"
        )
    fi
}

function set_chain_worker_args() {
    CHAIN_WORKER_ARGS=()
    if [[ "${CHAIN_NAME}" == "anvil" ]]; then
        CONFIRMATIONS=${CONFIRMATIONS:-1}
        if [[ "${EXAMPLE_NAME:-}" == "simple-time-travel" ]]; then
            CHAIN_WORKER_ARGS=(
                "http://localhost:8545 31337"
            )
        fi
    else
        CONFIRMATIONS=${CONFIRMATIONS:-1}
        if [ "${EXAMPLE_NAME:-}" == "simple-time-travel" ]; then
            CHAIN_WORKER_ARGS+=(
                "https://${QUICKNODE_ENDPOINT}.optimism-sepolia.quiknode.pro/${QUICKNODE_API_KEY} 11155420"
            )
        fi
    fi
}
