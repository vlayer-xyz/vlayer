# Dev & Production Modes

The vlayer node is an HTTP server that acts as a prover and supports two proving modes:
- **FAKE**: Used for development and testing. It executes code and verifies the correctness of execution but does not perform actual proving. In this mode, the `Verifier` contract can confirm computations, but a malicious `Prover` could exploit the system.
- **GROTH16**: Intended for production and final testing, this mode performs real proving.

By default, the vlayer client SDK communicates with `http://127.0.0.1:3000`.

## Running prover
Assuming vlayer is [installed](/getting-started/installation.html), you can start it in development mode with the following command:
```sh
vlayer serve
```

The vlayer prover server require urls of RPC node providers to query blockchain data. You can pass specific RPC URLs for each chain using the `--rpc-url` parameter:
```sh
vlayer serve --rpc-url <chain-id>:<url>
```

To configure multiple RPC URLs use `--rpc-url` parameter many times:
```sh
vlayer serve \
  --rpc-url 1:https://eth-mainnet.alchemyapi.io/v2/<alchemy_api_key> \
  --rpc-url 10:https://opt-mainnet.g.alchemy.com/v2/<optimism_api_key> 
```

> Note: By default, no RPC node providers are configured. You will need to specify them manually using the --rpc-url parameter to run the vlayer prover.

## FAKE Mode

By default, it listens for JSON-RPC client requests on port `3000` in `FAKE` mode. You can also specify the `--proof` argument explicitly:
```sh
vlayer serve --proof fake
```
See the [JSON-RPC API appendix](/appendix/api.md) for detailed specifications on API calls.
> Note: FAKE mode is limited to test and dev chains to prevent accidental errors.

## GROTH16 Mode
`GROTH16` mode is slower than `FAKE` mode and requires significant computational resources. 

To speed up proof generation, vlayer supports the use of infrastructure like the [Bonsai](https://www.bonsai.xyz/) (and eventually [Boundless](https://beboundless.xyz/)) to offload heavy computations to high-performance machines.

To run a vlayer node in production mode, use this command:

```sh
BONSAI_API_URL=https://api.bonsai.xyz/ \
BONSAI_API_KEY={api_key_goes_here} \
vlayer serve --proof groth16
```

You can request a `BONSAI_API_KEY` [here](https://docs.google.com/forms/d/e/1FAIpQLSf9mu18V65862GS4PLYd7tFTEKrl90J5GTyzw_d14ASxrruFQ/viewform).

> Note: Protocols should be designed with proving execution times in mind, as it may take a few minutes to generate proof.
