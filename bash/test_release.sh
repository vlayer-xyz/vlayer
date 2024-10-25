#!/usr/bin/env bash

set -ueo pipefail

echo '::group::foundry installation'
curl -L https://foundry.paradigm.xyz | bash
export PATH="$PATH:$HOME/.config/.foundry/bin"
foundryup
echo '::endgroup::'

echo '::group::vlayer installation'
curl -SL https://install.vlayer.xyz | bash
export PATH="$PATH:$HOME/.config/.vlayer/bin"
vlayerup
echo '::endgroup::'

echo '::group::risczero installation'
curl -L https://risczero.com/install | bash
export PATH="$PATH:$HOME/.risc0/bin"
rzup install cargo-risczero v1.0.5
echo '::endgroup::'

echo '::group::bun installation'
curl -fsSL https://bun.sh/install | bash
export PATH="$PATH:~/.bun/bin"
echo '::endgroup::'

git config --global user.email "test@example.com"
git config --global user.name "Github Runner"

echo '::group::vlayer template initialization'
mkdir ${HOME}/web_proof_test
cd ${HOME}/web_proof_test

vlayer init --template web-proof
forge build

cd vlayer
bun install
echo '::endgroup::'
