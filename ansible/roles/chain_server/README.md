# Vlayer Chain Server Ansible Role

Installs the vlayer Chain Server service.

## Variables

| Name | Purpose |
| --- | --- |
| `vlayer_release_channel` | Stable or nightly release channel. |
| `chain_server_rust_log` | An array of log levels for constructing [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
| `chain_server_host` | Host to bind to, for example `127.0.0.1` or `0.0.0.0`. |
| `chain_server_port` | A port on which the chain server should listen. |
| `chain_server_db_path` | Where is the DB located. |
