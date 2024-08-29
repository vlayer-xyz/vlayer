#!/usr/bin/env bash

set -uexo pipefail

function create_tmp_dir() {
        cd $(mktemp -d)
}

function test_can_initialise_properly() {
    (    
        create_tmp_dir

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

function test_can_initialise_an_existing_project() {
    (    
        create_tmp_dir
        forge init myproject
        cd myproject

        vlayer init --existing
    
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
        create_tmp_dir

        vlayer init myproject

        set +o pipefail # vlayer command will fail, so we need to turn off pipefail option for the next expression
        vlayer init myproject | grep -q "ERROR" # should log an error. If not, 'grep' exits with 1
    )
}

####### SETUP

curl -SL  https://install.vlayer.xyz | bash
source  "${HOME}/.bashrc"
vlayerup

####### TESTS
test_can_initialise_properly
test_can_initialise_an_existing_project
test_init_is_not_idempotent
