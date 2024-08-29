#!/usr/bin/env bash

set -uexo pipefail

function setup_foundry_project() {
        cd $(mktemp -d)
}

function test_can_initialise_properly() {
    (    
        setup_foundry_project

        vlayer init myproject
        cd myproject
    
        contracts=(
            "SimpleProver.sol"
            "SimpleProver.t.sol"
            "SimpleVerifier.sol"
        )
    
        for contract in "${contracts[@]}"; do
            if [[ ! -f "src/vlayer/${contract}" ]] ; then
                echo "ERROR: $contract not found" >&2
                exit 1
            fi
        done
    )
}

function test_init_is_not_idempotent() {
    (    
        setup_foundry_project

        vlayer init myproject

        # should log an error. If not, 'grep' exits with 1
        vlayer init myproject | grep -q "ERROR" 
    )
}

####### SETUP

curl -SL  https://install.vlayer.xyz | bash
source  "${HOME}/.bashrc"
vlayerup

####### TESTS
test_can_initialise_properly
test_init_is_not_idempotent
