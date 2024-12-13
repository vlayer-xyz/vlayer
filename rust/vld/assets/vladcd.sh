vladcd() {
    local result
    result="$(vlad "$@")"
    if [ $? -eq 0 ] && [ -n "$result" ] && [ -d "$result" ]; then
        cd "$result"
    else
        echo "Navigation failed"
        return 1
    fi
}
