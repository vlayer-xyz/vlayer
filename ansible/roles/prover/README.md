# Vlayer Prover Ansible Role

Installs the vlayer Prover server.

## Variables

| Name | Purpose |
| --- | --- |
| `vlayer_prover_host` | Host to bind to, for example `127.0.0.1` or `0.0.0.0`. |
| `vlayer_prover_port` | The port to bind to. |
| `vlayer_prover_rpc_urls` | A list of RPC urls for the Prover. |
| `vlayer_proof_type` | Type of proof - `fake` or `groth16`. |
