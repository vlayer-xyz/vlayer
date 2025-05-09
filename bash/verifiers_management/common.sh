VLAYER_HOME=$(git rev-parse --show-toplevel)
CONTRACTS_DIR="${VLAYER_HOME}/contracts/vlayer"

DRY_RUN=${DRY_RUN:-true}

echo "DRY_RUN: ${DRY_RUN}"

function broadcast_flag(){
  if [[ "${DRY_RUN}" == "false" ]]; then
      echo "--broadcast"
  fi
}

function cleanup(){
  echo "Cleaning up artifacts..."
  if [[ -d "./broadcast" ]] ; then 
    rm -r ./broadcast ./cache
  fi
  forge clean

}

function build_contracts(){
  echo "Building contracts..."
  forge build 
}

function run_forge_script(){
  local script_invocation=$1
  local network=$2

  echo "Running script on ${network}"
  forge script "${script_invocation}" \
    --rpc-url "${network}" \
    --slow \
    "$(broadcast_flag)" 
}
