# Vlayer Chain Service Docker Ansible Role

Installs the vlayer Chain Service docker containers (1 chain server + multiple chain workers).

## Variables

| Name | Purpose |
| --- | --- |
| `chain_service_docker_version` | A docker image tag to use. |
| `chain_service_docker_server_host` | Host to bind to for chain server, for example `127.0.0.1` or `0.0.0.0`. |
| `chain_service_docker_server_port` | The port to bind to on host system for chain server. |
| `chain_service_docker_workers` | Array of chain worker configurations with blockchain settings. |
| `chain_service_docker_server_rust_log` | An array of log levels for chain server [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
| `chain_service_docker_worker_rust_log` | An array of log levels for chain workers [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
| `chain_service_docker_db_path` | Database path for containers (inside container). |
