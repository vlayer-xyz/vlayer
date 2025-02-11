function set_proving_mode() {
  PROVING_MODE=${PROVING_MODE:-dev}

  if [[ "$PROVING_MODE" == "dev" ]]; then
    export RISC0_DEV_MODE=1
  else
    unset RISC0_DEV_MODE
  fi
}