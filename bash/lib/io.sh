# The behavior of this function is controlled by the VERBOSE_MODE environment variable.
# If VERBOSE_MODE is set to "true", the command's output will be printed.
# If VERBOSE_MODE is not set or is "false", the command's output will be silenced unless it fails.

function silent_unless_fails() {
    echo "Running: $@"

    # Check if VERBOSE_MODE is enabled
    if [[ "${VERBOSE_MODE:-false}" == "true" ]]; then
        # Execute the command and print all output
        "$@"
    else
        # Create a temporary file for capturing output
        local tmp_file
        tmp_file=$(mktemp)

        # This prevents risc0-build from printing directly to tty even if we redirect stdout and stderr
        export RISC0_GUEST_LOGFILE="$tmp_file"

        function silent_unless_fails_cleanup() {
            unset RISC0_GUEST_LOGFILE
            rm -f "$tmp_file"
        }

        # Execute the command and redirect both stdout and stderr to the temporary file
        if ! "$@" >"$tmp_file" 2>&1; then
            echo "Command failed: $@"
            cat "$tmp_file"
            silent_unless_fails_cleanup
            return 1
        fi

        silent_unless_fails_cleanup
    fi
}
