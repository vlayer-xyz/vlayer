# Vlayer Prover Nginx Ansible Role

Installs and configures Nginx in front of the prover application.

## Variables

| Name | Purpose |
| --- | --- |
| `vlayer_prover_port` | The port on which the latest prover application binary listens. |
| `prover_nginx_default_prover_port` | Which port nginx should redirect to for a default prover version under the main URL. |
| `prover_nginx_ip_rate_limit_per_minute` | How many requests are allowed per IP per minute. |
| `prover_nginx_ip_rate_limit_burst` | Allowed burst in rate limiting. |
| `prover_nginx_ssl_certificate` | Which SSL certificate to use. |
