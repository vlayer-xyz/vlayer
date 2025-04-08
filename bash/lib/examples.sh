# You can set the $EXAMPLE variable to run a single example
function get_examples() {
    local EXAMPLE_LIST=(
        "simple"
        "simple-email-proof"
        "simple-web-proof"
        "simple-time-travel"
        "simple-teleport"
        "kraken-web-proof"
    )

    # "simple-teleport" is not enabled on testnet as we still need to deploy chain workers
    if [[ -n ${VLAYER_ENV:-} ]]; then
        if [[ "$VLAYER_ENV" == "testnet" ]]; then
            EXAMPLE_LIST=("${EXAMPLE_LIST[@]/simple-teleport}")
        fi
    fi

    if [[ -n ${EXAMPLE:-} ]]; then
        if ! [[ " ${EXAMPLE_LIST[*]} " == *" $EXAMPLE "* ]]; then
            echo "Error: Invalid EXAMPLE_NAME '$EXAMPLE'. Valid options are: ${EXAMPLE_LIST[*]}" >&2
            exit 1
        fi
        echo $EXAMPLE
    else
        echo "${EXAMPLE_LIST[@]}"
    fi
}
