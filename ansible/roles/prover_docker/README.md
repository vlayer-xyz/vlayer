# Vlayer Prover Docker Ansible Role

Installs the vlayer Prover docker container.

## Variables

| Name | Purpose |
| --- | --- |
| `prover_docker_version` | A docker image tag to use. |
| `prover_docker_host` | Host to bind to, for example `127.0.0.1` or `0.0.0.0`. |
| `prover_docker_port` | The port to bind to on host system. |
| `prover_docker_proof_type` | Type of proof - `fake` or `groth16`. |
| `prover_docker_rust_log` | An array of log levels for constructing [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
| `prover_docker_bonsai_api_url` | API url for Bonsai, required for real proofs. |
| `prover_docker_bonsai_api_key` | API key for Bonsai, required for real proofs. |
| `prover_docker_gas_meter_url` | Optional url to the gas meter endpoint. |
| `prover_docker_gas_meter_api_key` | API key for the gas meter endpoint. |
| `prover_docker_chain_proof_url` | Optional url to the chain proof server. |
| `prover_docker_rpc_urls` | A list of RPC urls for the Prover. |
| `prover_docker_jwt_algorithm` | Algorithm type used in JWT.. |
| `prover_docker_jwt_claims` | A list of JWT claims. |
| `prover_docker_jwt_public_key_location` | Where is the JWT public key file installed. |
| `prover_docker_optimism_sepolia_rollup_endpoint` | Optional override URL for Optimism Sepolia rollup node endpoint. |
