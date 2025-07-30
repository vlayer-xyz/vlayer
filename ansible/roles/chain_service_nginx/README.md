# Vlayer Chain Service Nginx Ansible Role

Installs and configures Nginx in front of the chain service.

## Variables

| Name | Purpose |
| --- | --- |
| `chain_server_port` | The port on which the latest chain server application binary listens. |
| `chain_service_nginx_default_server_port` | Which port nginx should redirect to for a default chain server version under the main URL. |
| `chain_service_nginx_ip_rate_limit_per_minute` | How many requests are allowed per IP per minute. |
| `chain_service_nginx_ip_rate_limit_burst` | Allowed burst in rate limiting. |
