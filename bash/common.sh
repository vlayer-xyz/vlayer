ensure_cmd() {
  if ! check_cmd "$1"; then
    echo "Command not found: '$1'" >&2
  fi
}

check_cmd() {
  command -v "$1" &>/dev/null
}

check_exit_status() {
  if [ $? -ne 0 ]; then
    echo "$1"
    exit 1
  fi
}

kill_service() {
  service_name=$1

  echo "Killing ${service_name}..." 
  service_pid=${!service_name:-}

  if [[ -n "${service_pid}" ]] && ps -p "${service_pid}" >/dev/null; then
      echo "Killing ${service_name} (PID ${service_pid})..."
      kill "${service_pid}"
  else
      echo "Service ${service_name} is not alive. Skipping."
  fi
}

wait_for_port_and_pid() {
    local port=$1
    local pid=$2
    local timeout=$3
    local service_name=$4

    ensure_cmd timeout

    echo "Waiting for ${service_name} to be ready on localhost:${port}..."

    # wait until port is open and the expected pid is alive
    # if the port is open, but pid is not alive, exit 
    timeout >/dev/null 2>&1 --preserve-status --foreground --kill-after=5s "${timeout}" bash -c \
        "sleep 3 ;  while ! (nc -z localhost ${port}) && ps -p $pid ; do  sleep 3;  done ; if ! (ps -p $pid) ; then exit 1 ; fi"
    
    check_exit_status "Error: Timeout reached. ${service_name} is not available on localhost:${port}."
}

function startup_anvil() {
    local LOG_DIR=$1
    local PORT=$2
    local ANVIL_VAR_NAME=$3

    ensure_cmd anvil
    anvil -p ${PORT} > ${LOG_DIR} &
    ANVIL_PID=$!
    echo "Anvil started with PID ${ANVIL_PID}."
    wait_for_port_and_pid ${PORT} ${ANVIL_PID} 30s "Anvil"
    echo "Anvil running on port ${PORT}"
    
    eval "${ANVIL_VAR_NAME}=${ANVIL_PID}"
}
