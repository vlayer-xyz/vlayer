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

## Variable Secrets

Host variables may contain sensitive information, like secret API keys.

### Add a new secret variable

1. `ansible-vault encrypt_string --name '<string_name_of_variable>'`
2. Add the output to the destination host var file.

[Reference](https://docs.ansible.com/ansible/latest/vault_guide/index.html)

## Running playbooks

1. Prover playbook

```sh
ansible-playbook -i hosts.yml prover.yml --ask-vault-pass
```

2. Github runner playbook

This requires the `PERSONAL_ACCESS_TOKEN` github token with `repo:write` access rights,
in order to register repository runners.

```sh
PERSONAL_ACCESS_TOKEN='xxx' ansible-playbook -i ../terraform/github-runners.ini github_runners.yml
```
