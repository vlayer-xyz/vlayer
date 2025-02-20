VLAYER_HOME=$(git rev-parse --show-toplevel)
CONTRACTS_DIR="${VLAYER_HOME}/contracts/vlayer"

DRY_RUN=${DRY_RUN:-true}
NETWORKS=( "base" "sepolia" "base-sepolia" "optimism-sepolia" "arbitrum-sepolia" "worldchain-sepolia" )

echo "DRY_RUN: ${DRY_RUN}"
echo "NETWORKS: ${NETWORKS[@]}"

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

  for NETWORK in "${NETWORKS[@]}" ; do
    echo "Running script on ${NETWORK}"
    forge script "${script_invocation}" \
      --rpc-url "${NETWORK}" \
      --slow \
      "$(broadcast_flag)" 
  done
}
