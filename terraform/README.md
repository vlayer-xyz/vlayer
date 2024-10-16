# Terraform

This is a collection of Terraform modules and configurations for infrastracture.

## Prerequisites

1. [Install AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html).
2. [Install Terraform](https://docs.ansible.com/ansible/latest/installation_guide/intro_installation.html).

## State

The Terraform state is [stored remotely](https://developer.hashicorp.com/terraform/language/state/remote) in S3 with locks.

## Applying changes

First, run `terraform plan` (a dry run).

Next, run `terraform apply` to apply planned changes.
