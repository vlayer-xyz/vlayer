# Vlayer Verifiable DNS Nginx Ansible Role

Installs and configures Nginx in front of the verifiable_dns application.

## Variables

| Name | Purpose |
| --- | --- |
| `verifiable_dns_port` | The port on which the verifiable_dns application listens. |
| `verifiable_dns_nginx_ip_rate_limit_per_minute` | How many requests are allowed per IP per minute. |
| `verifiable_dns_nginx_ip_rate_limit_burst` | Allowed burst in rate limiting. |
