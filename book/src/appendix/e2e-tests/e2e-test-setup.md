# E2E Testing Setup

## Overview

This document describes the setup phase of end-to-end (E2E) testing for the vlayer system as implemented in the `e2e-test.sh` script. The setup phase initializes all necessary services, configurations, and components required for testing.

## Key Components

### 1. Environment Configuration

The setup phase begins by configuring the environment:

```bash
# Environment variables setup
VLAYER_HOME=$(git rev-parse --show-toplevel)
export PATH="${VLAYER_HOME}/target/debug:${PATH}"

# Load environment variables
[ -f "${VLAYER_HOME}/.env.local" ] && source "${VLAYER_HOME}/.env.local"

# Set default values
VLAYER_ENV=${VLAYER_ENV:-dev}
BUILD_CLI=${BUILD_CLI:-1}
```

- `VLAYER_HOME` points to the root directory of the vlayer project
- Environment variables from `.env.local` are loaded if the file exists
- Default values are set for environment-specific configurations

### 2. Helper Scripts

The setup phase sources several helper scripts that provide utility functions:

```bash
source "$(dirname "${BASH_SOURCE[0]}")/common.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/examples.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/proving_mode.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/e2e.sh"
source "$(dirname "${BASH_SOURCE[0]}")/lib/build-packages.sh"
```

These scripts provide functions for the following stages of e2e testing:

1. **Environment Setup:**

   - [`set_proving_mode`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/proving_mode.sh): Configures proving mode (dev/prod)
   - [`generate_ts_bindings`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/utils.sh): Generates TypeScript bindings for contracts
   - [`build_sdk`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/build-packages.sh): Builds the SDK package

2. **Service Initialization:**

   - [`ensure_services_built`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/lib.sh): Compiles the required service binaries (call_server, chain_server, worker, dns_server) if `BUILD_SERVICES` equals `1`
   - [`startup_chain_worker`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/chain_worker.sh): Starts chain worker processes
   - [`startup_chain_server`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/lib.sh): Starts the chain server for RPC communication
   - [`startup_vlayer`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/lib.sh): Starts the vlayer REST server
   - [`startup_vdns_server`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/lib.sh): Starts the DNS server
   - [`startup_chain_services`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/lib.sh): Coordinates starting all chain-related services
   - [`wait_for_port_and_pid`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/common.sh): Waits for services to be ready on specific ports

   The service initialization follows a specific sequence: first `ensure_services_built` compiles all service binaries, then Docker services are started, followed by chain workers, chain server, vlayer server, and finally the DNS server.

3. **Test Environment Preparation:**

   - [`generate_vlayer_init_config`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh): Creates configuration for vlayer initialization
   - [`ensure_cli_built`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh): Ensures the CLI is built
   - [`init_template`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh): Initializes the test template

4. **Test Execution:**

   - [`silent_unless_fails`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/io.sh): Controls output of command execution
   - [`run_prover_script`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh): Runs the prover script

5. **Cleanup:**
   - [`cleanup`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/run_services/cleanup.sh): Cleans up all services and temporary files
   - [`kill_service`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/common.sh): Terminates specific services by PID

### 3. Proving Mode Configuration

```bash
set_proving_mode
```

The proving mode determines how proofs are generated and verified:

- `dev` mode uses fake proofs for faster testing
- `prod` mode uses production-grade proofs (Bonsai)

This configuration affects both the server and worker components:

- Sets `SERVER_PROOF_ARG` (fake/groth16)
- Sets `WORKER_PROOF_ARG` (fake/succinct)

### 4. TypeScript Bindings and SDK Generation

```bash
generate_ts_bindings
build_sdk
```

These steps ensure that:

- The TypeScript bindings for smart contracts are up-to-date
- The SDK is built with the latest changes

### 5. Service Startup

```bash
DOCKER_COMPOSE_SERVICES="anvil-l1 anvil-l2-op notary-server"
source ${VLAYER_HOME}/bash/run-services.sh
```

This starts the required services:

- `anvil-l1`: Local Ethereum L1 node
- `anvil-l2-op`: Local Optimism L2 node
- `notary-server`: Notary server

The service startup orchestrates:

1. Docker containers for blockchain nodes
2. Chain workers for precomputing chain proofs
3. Chain server for making chain proofs available over JSON-RPC
4. vlayer server for call proof generation

### 6. Chain Worker Configuration

Chain workers are configured based on the test environment:

- In devnet (Anvil), the worker connects to `http://localhost:8545` with chain ID `31337`
- In testnet, the worker connects to external RPC endpoints with their respective chain IDs

Workers are started with parameters:

```bash
RUST_LOG=${RUST_LOG:-info} ./target/debug/worker \
    --db-path "${db_path}" \
    --rpc-url "${rpc_url}" \
    --chain-id "${chain_id}" \
    --proof-mode "${WORKER_PROOF_ARG}" \
    --confirmations "${CONFIRMATIONS:-1}" \
    --max-head-blocks "${MAX_HEAD_BLOCKS:-10}" \
    --max-back-propagation-blocks "${MAX_BACK_PROPAGATION_BLOCKS:-10}"
```

### 7. Test Environment Preparation

```bash
cd $(mktemp -d)
generate_vlayer_init_config
ensure_cli_built
init_template
```

These steps:

1. Create a temporary directory for testing
2. Generate the vlayer initialization configuration ([`generate_vlayer_init_config`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh))
3. Ensure the CLI tool is built ([`ensure_cli_built`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh))
4. Initialize a test template with the example code ([`init_template`](http://github.com/vlayer-xyz/vlayer/blob/main/bash/lib/e2e.sh))

## Key Configurables

The following environment variables can be passed to the `e2e-test.sh` script to customize its behavior:

| Variable                      | Default        | Description                                                                                              | Passed to Script? |
| ----------------------------- | -------------- | -------------------------------------------------------------------------------------------------------- | ----------------- |
| `VLAYER_ENV`                  | `dev`          | Environment type (dev/prod)                                                                              | ✓                 |
| `PROVING_MODE`                | `dev`          | Proving mode (dev/prod)                                                                                  | ✓                 |
| `BUILD_SERVICES`              | `1`            | Controls whether service binaries are compiled; set to `0` to skip compilation if binaries already exist | ✓                 |
| `BUILD_CLI`                   | `1`            | Whether to build the CLI                                                                                 | ✓                 |
| `EXAMPLE`                     | _Required_     | Name of the example to test (must be set)                                                                | ✓                 |
| `CONFIRMATIONS`               | `1`            | Number of confirmations required for chain workers                                                       | ✓                 |
| `MAX_HEAD_BLOCKS`             | `10`           | Maximum head blocks to process in chain workers                                                          | ✓                 |
| `MAX_BACK_PROPAGATION_BLOCKS` | `10`           | Maximum back propagation blocks in chain workers                                                         | ✓                 |
| `VLAYER_TMP_DIR`              | auto-generated | Directory for temporary files                                                                            | ✓                 |
| `CHAIN_NAME`                  | `anvil`        | Chain to use (anvil or a testnet like optimismSepolia)                                                   | ✓                 |

For prod mode with external chains, these additional variables are required:

| Variable             | Default                   | Description            | Required When       |
| -------------------- | ------------------------- | ---------------------- | ------------------- |
| `BONSAI_API_URL`     | `https://api.bonsai.xyz/` | URL for Bonsai API     | `PROVING_MODE=prod` |
| `BONSAI_API_KEY`     | None                      | API key for Bonsai     | `PROVING_MODE=prod` |
| `QUICKNODE_API_KEY`  | None                      | API key for QuickNode  | `CHAIN_NAME!=anvil` |
| `QUICKNODE_ENDPOINT` | None                      | Endpoint for QuickNode | `CHAIN_NAME!=anvil` |
