RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
BLUE='\033[34m'
NC='\033[0m' # No Color

function echo_color() {
    local color_name="$1"
    local text="$2"

    if [[ -n "${NO_COLOR:-}" ]]; then
        echo -e "${text}"
    else
        shift
        echo -e "${!color_name}${text}${NC}"
    fi
}
