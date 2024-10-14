# Dev & Production Modes

The vlayer node supports two proving modes:

- **DEVELOPMENT**: Used for development and testing. It executes code and verifies the correctness of execution but does not perform actual proving. In this mode, the `Verifier` contract can confirm computations, but a malicious `Prover` could exploit the system.
- **PRODUCTION**: Intended for production and final testing, this mode performs real proving.

## Development Mode

By default, the vlayer node runs in `DEVELOPMENT` mode:

```sh
vlayer serve
```

You can also explicitly provide the `--proof` argument:

```sh 
vlayer serve --proof fake
```

> Development mode is restricted to test and dev chains to avoid accidental errors.

## Production Mode

To run a vlayer node in production mode, use the following command:

```sh
BONSAI_API_URL=https://api.bonsai.xyz/ \
BONSAI_API_KEY={api_key_goes_here} \
vlayer serve --proof groth16
```

Two environment variables (`BONSAI_API_URL` and `BONSAI_API_KEY`) are needed enable highly parallelized and efficient proof generation using the [Bonsai zk coprocessor](https://www.bonsai.xyz/).

API keys can be [requested here](https://docs.google.com/forms/d/e/1FAIpQLSf9mu18V65862GS4PLYd7tFTEKrl90J5GTyzw_d14ASxrruFQ/viewform).

> Production mode is much slower than development mode. Protocols should be designed with proving execution time in mind, as it can take several minutes.