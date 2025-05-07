# Vlayer Chain Worker Ansible Role

Installs the vlayer Chain Worker service.

Typically, more than 1 chain worker would be installed on a single machine.

## Variables

| Name | Purpose |
| --- | --- |
| `vlayer_release_channel` | Stable or nightly release channel. |
| `chain_worker_identifier` | An identifier for distinguishing multiple workers running on a single machine. |
| `chain_worker_rust_log` | An array of log levels for constructing [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
| `chain_worker_db_path` | Where is the DB located. |
| `chain_worker_rpc_url` | Blockchain RPC URL |
| `chain_worker_chain_id` | ID of the chain to index |
| `chain_worker_proof_mode` | Proof generation mode |
| `chain_worker_bonsai_api_url` | Bonsai API URL |
| `chain_worker_bonsai_api_key` | Bonsai API key |
| `chain_worker_max_back_propagation_blocks` | Maximum number of historical blocks prepended in a single batch |
| `chain_worker_max_head_blocks` | Maximum number of new blocks appended in a single batch |
| `chain_worker_confirmations` | Minimum number of confirmations required for a block to be appended |
