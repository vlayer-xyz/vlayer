#!/usr/bin/env bash

set -ueo pipefail

curl -L https://foundry.paradigm.xyz | bash
export PATH="$PATH:/home/runner/.config/.foundry/bin"
foundryup

curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:/home/runner/.config/.vlayer/bin"
vlayerup

curl -L https://risczero.com/install | bash
export PATH="$PATH:/home/runner/.risc0/bin"
rzup install cargo-risczero v1.0.5

curl -fsSL https://bun.sh/install | bash
export PATH="$PATH:~/.bun/bin"

git config --global user.email "test@example.com"
git config --global user.name "Github Runner"

mkdir /home/runner/web_proof_test
cd /home/runner/web_proof_test

vlayer init --template web-proof
forge build

cd vlayer
bun install
