# Vlayer Prover Ansible Role

Installs the vlayer Prover server.

## Variables

| Name | Purpose |
| --- | --- |
| `vlayer_prover_host` | Host to bind to, for example `127.0.0.1` or `0.0.0.0`. |
| `vlayer_prover_port` | The port to bind to. |
| `vlayer_release_channel` | Stable or nightly release channel. |
| `vlayer_prover_rpc_urls` | A list of RPC urls for the Prover. |
| `vlayer_prover_gas_meter_url` | Optional url to the gas meter endpoint. |
| `vlayer_prover_chain_proof_url` | Optional url to the chain proof server. |
| `vlayer_proof_type` | Type of proof - `fake` or `groth16`. |
| `vlayer_bonsai_api_url` | API url for Bonsai, required for real proofs. |
| `vlayer_bonsai_api_key` | API key for Bonsai, required for real proofs. |
| `vlayer_jwt_public_key_location` | Where to install the JWT public key file. |
| `vlayer_rust_log` | An array of log levels for constructing [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
