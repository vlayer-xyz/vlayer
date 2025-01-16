# You can set the $EXAMPLE variable to run a single example
function get_examples() {
    local EXAMPLE_LIST=(
        "simple"
        "simple_email_proof"
        "simple_web_proof"
        "simple_time_travel"
        "simple_teleport"
    )

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

# You can set the $TEMPLATE variable to run a single example
function get_templates() {
    local TEMPLATE_LIST=(
        "simple"
        "simple-email-proof"
        "simple-web-proof"
        "simple-time-travel"
        "simple-teleport"
    )

    if [[ -n ${TEMPLATE:-} ]]; then
        if ! [[ " ${TEMPLATE_LIST[*]} " == *" $TEMPLATE"* ]]; then
            echo "Error: Invalid TEMPLATE_NAME '$TEMPLATE'. Valid options are: ${TEMPLATE_LIST[*]}" >&2
            exit 1
        fi
        echo $TEMPLATE
    else
        echo "${TEMPLATE_LIST[@]}"
    fi
}
