# Prover

The vlayer node is an HTTP server that acts as a prover. You can start it with the following command:

```sh
vlayer serve
```

By default, it accepts JSON-RPC requests on port `3000` in the format specified in the [JSON-RPC API appendix](/appendix/api.md).

## Proving Modes

The vlayer node provides two proving modes:

- `DEVELOPMENT` - For development and testing only. It executes code and verifies the correctness of the execution, but doesn't perform any actual proving. In this mode, the `Verifier` contract verifies the correctness of computations, but it can be cheated by a malicious `Prover`.
- `PRODUCTION` - Intended for production and final testing. It performs the actual proving.

> By default, the vlayer node operates in the `DEVELOPMENT` mode.
> Note that the `PRODUCTION` mode is much slower than the `DEVELOPMENT` mode. It is important to design protocols with proving execution time in mind, which takes at least a few minutes.
> The `DEVELOPMENT` mode only works on development and test chains to avoid accidental errors.
