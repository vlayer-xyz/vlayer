# Ansible

This is a collection of vlayer ansible scripts and roles.

## Prerequisites

1. [Install Ansible](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html).
2. Install galaxy collections: `ansible-galaxy collection install -r requirements.yml`
3. Install galaxy roles: `ansible-galaxy role install -r requirements.yml`

## Available roles

| Role | Purpose |
| --- | --- |
| prover | Installs a vlayer Prover server. [Readme](./roles/prover/) |
| verifiable_dns | Installs a vlayer verifiable dns service. [Readme](./roles/verifiable_dns/) |

## Variable Secrets

Host variables may contain sensitive information, like secret API keys.

### Add a new secret variable

1. `ansible-vault encrypt_string --name '<string_name_of_variable>'`
2. Add the output to the destination host var file.

[Reference](https://docs.ansible.com/ansible/latest/vault_guide/index.html)

## Running playbooks

Currently, there is one playbook for installing the Prover.

To run it:

```sh
ansible-playbook -i hosts.yml prover.yml --ask-vault-pass
```
