# Vlayer Prover Nginx Ansible Role

Installs and configures Nginx in front of the prover application.

## Variables

| Name | Purpose |
| --- | --- |
| `vlayer_prover_port` | The port on which the prover application listens. |
| `prover_nginx_ip_rate_limit_per_minute` | How many requests are allowed per IP per minute. |
| `prover_nginx_ip_rate_limit_burst` | Allowed burst in rate limiting. |
| `prover_nginx_ssl_certificate` | Which SSL certificate to use. |
