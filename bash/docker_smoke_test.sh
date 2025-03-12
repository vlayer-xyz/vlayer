TIMEOUT=60
INTERVAL=5
END=$((SECONDS+TIMEOUT))
DOTS=""

while [ $SECONDS -lt $END ]; do
    UNHEALTHY=$(docker ps --filter health=unhealthy --format "{{.Names}}" | wc -l)
    STARTING=$(docker ps --filter health=starting --format "{{.Names}}" | wc -l)
    HEALTHY=$(docker ps --filter health=healthy --format "{{.Names}}" | wc -l)
    TOTAL=$(docker ps --format "{{.Names}}" | wc -l)
    NO_HEALTHCHECK=$(docker ps --filter health=none --format "{{.Names}}" | wc -l)

    # Move cursor to beginning of line and clear line
    echo -en "\r\033[K"
    
    echo -e "Health Status - \033[32mHEALTHY: $HEALTHY\033[0m | \033[34mSTARTING: $STARTING\033[0m | \033[31mUNHEALTHY: $UNHEALTHY\033[0m | \033[33mNO HEALTHCHECK: $NO_HEALTHCHECK\033[0m | TOTAL: $TOTAL"

    if [[ "$UNHEALTHY" -gt 0 ]]; then
        echo_color RED "\nUnhealthy services:"
        docker ps --filter health=unhealthy --format "{{.Names}}"
    fi

    if [[ "$HEALTHY" -eq $((TOTAL-NO_HEALTHCHECK)) ]]; then
        echo_color GREEN "\n✓ All services with health checks are healthy!"
        exit 0
    fi

    DOTS="${DOTS}."
    if [ ${#DOTS} -gt 3 ]; then
        DOTS=""
    fi
    echo_color YELLOW "\nWaiting for services to become healthy${DOTS}"
    sleep $INTERVAL
done

UNHEALTHY_SERVICES=$(docker ps --filter health=unhealthy --format "{{.Names}}")
STARTING_SERVICES=$(docker ps --filter health=starting --format "{{.Names}}")

echo_color RED "\nError: Services not healthy after $TIMEOUT seconds." >&2
[ -n "$UNHEALTHY_SERVICES" ] && echo -e "\033[31mUnhealthy services:\033[0m\n$UNHEALTHY_SERVICES" >&2
[ -n "$STARTING_SERVICES" ] && echo -e "\033[34mStill starting:\033[0m\n$STARTING_SERVICES" >&2
echo "Exiting..."
exit 1 
