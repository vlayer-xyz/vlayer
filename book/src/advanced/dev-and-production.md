# Dev & Production Modes

The vlayer node is an HTTP server that acts as a prover and supports two proving modes:
- **DEVELOPMENT**: Used for development and testing. It executes code and verifies the correctness of execution but does not perform actual proving. In this mode, the `Verifier` contract can confirm computations, but a malicious `Prover` could exploit the system.
- **PRODUCTION**: Intended for production and final testing, this mode performs real proving.

The vlayer prover node can be run locally, but it is not mandatory. By default, the vlayer client SDK communicates with a pre-deployed prover running in development mode.

## Running the Prover Server
Assuming vlayer is [installed](/getting-started/installation.html), you can start it with the following command:
```sh
vlayer serve
```
By default, it listens for JSON-RPC client requests on port `3000`.

See the [JSON-RPC API appendix](/appendix/api.md) for detailed specifications on API calls.

### Development Mode
By default, the vlayer node runs in `DEVELOPMENT` mode:

```sh
vlayer serve
```

You can also specify the `--proof` argument explicitly:

```sh
vlayer serve --proof fake
```

> Note: Development mode is limited to test and dev chains to prevent accidental errors.

### Production Mode
Production mode is slower than development mode and requires significant computational resources. 

To speed up proof generation, vlayer supports the use of infrastructure like the [Bonsai zk coprocessor](https://www.bonsai.xyz/) (and eventually Boundless) to offload heavy computations to high-performance machines.

To run a vlayer node in production mode, use this command:

```sh
BONSAI_API_URL=https://api.bonsai.xyz/ \
BONSAI_API_KEY={api_key_goes_here} \
vlayer serve --proof groth16
```

You can request a `BONSAI_API_KEY` [here](https://docs.google.com/forms/d/e/1FAIpQLSf9mu18V65862GS4PLYd7tFTEKrl90J5GTyzw_d14ASxrruFQ/viewform).

> Note: Protocols should be designed with proving execution times in mind, as it may take several minutes to generate proofs.

## Network-Specific Configuration
The vlayer prover server can use different RPC node providers to query blockchain data. You can pass specific RPC URLs for each chain using the `rpc-url` parameter:
```sh
vlayer serve --rpc-url <chain-id>:<url>
```

You can also configure multiple RPC URLs at once:
```sh
vlayer serve --rpc-url 11155111:https://eth-sepolia.g.alchemy.com/v2/<mainnet_api_key> --rpc-url 1:https://eth-mainnet.alchemyapi.io/v2/<alchemy_api_key>
```
