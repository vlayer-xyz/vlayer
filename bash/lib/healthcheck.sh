# Define color variables globally
RED='\e[31m'
GREEN='\e[32m'
YELLOW='\e[33m'
BLUE='\e[34m'
NC='\e[0m' # No Color

check_services_health() {
    local TIMEOUT=60
    local INTERVAL=5
    local END=$((SECONDS+TIMEOUT))
    local DOTS=""

    while [ $SECONDS -lt $END ]; do
        local UNHEALTHY=$(docker ps --filter health=unhealthy --format "{{.Names}}" | wc -l)
        local STARTING=$(docker ps --filter health=starting --format "{{.Names}}" | wc -l)
        local HEALTHY=$(docker ps --filter health=healthy --format "{{.Names}}" | wc -l)
        local TOTAL=$(docker ps --format "{{.Names}}" | wc -l)
        local NO_HEALTHCHECK=$(docker ps --filter health=none --format "{{.Names}}" | wc -l)

        # Move cursor to beginning of line and clear line
        echo -en "\r\033[K"
        
        echo -en "Health Status - ${GREEN}HEALTHY: $HEALTHY${NC} | ${BLUE}STARTING: $STARTING${NC} | ${RED}UNHEALTHY: $UNHEALTHY${NC} | ${YELLOW}NO HEALTHCHECK: $NO_HEALTHCHECK${NC} | TOTAL: $TOTAL"

        if [[ "$UNHEALTHY" -gt 0 ]]; then
            echo -en "\n${RED}Unhealthy services:${NC}\n"
            docker ps --filter health=unhealthy --format "{{.Names}}"
        fi

        if [[ "$HEALTHY" -eq $((TOTAL-NO_HEALTHCHECK)) ]]; then
            echo -en "\n${GREEN}✓ All services with health checks are healthy!${NC}\n"
            exit 0
        fi

        DOTS="${DOTS}."
        if [ ${#DOTS} -gt 3 ]; then
            DOTS=""
        fi
        echo -en "\n${YELLOW}Waiting for services to become healthy${DOTS}${NC}"
        sleep $INTERVAL
    done

    local UNHEALTHY_SERVICES=$(docker ps --filter health=unhealthy --format "{{.Names}}")
    local STARTING_SERVICES=$(docker ps --filter health=starting --format "{{.Names}}")
    
    echo -e "\n${RED}Error: Services not healthy after $TIMEOUT seconds.${NC}" >&2
    [ -n "$UNHEALTHY_SERVICES" ] && echo -e "${RED}Unhealthy services:${NC}\n$UNHEALTHY_SERVICES" >&2
    [ -n "$STARTING_SERVICES" ] && echo -e "${BLUE}Still starting:${NC}\n$STARTING_SERVICES" >&2
    echo "Exiting..."
    exit 1 
}