# Vlayer Verifiable DNS Ansible Role

Installs the verifiable dns service.

## Variables

| Name | Purpose |
| --- | --- |
| `verifiable_dns_host` | Host to bind to, for example `127.0.0.1` or `0.0.0.0`. |
| `verifiable_dns_port` | The port to bind to. |
| `verifiable_dns_rust_log` | An array of log levels for constructing [`RUST_LOG`](https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html). |
