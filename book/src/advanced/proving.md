# Prover

The vlayer node is an HTTP server that acts as a prover. You can start it with the following command:

```sh
vlayer serve
```

By default, it accepts HTTP requests on port `3000` in the format specified in the [JSON-RPC API appendix](/appendix/api.md).

## Proving Modes

The vlayer node provides two proving modes:

- `DEV` - Intended for development and testing only. It executes code and verifies the correctness of execution but doesn't perform actual proving. In this mode, the Verifier contract will check the correctness of computations, but it can be cheated by a malicious prover.
- `PRODUCTION` - Intended for production and final testing. It performs actual proving.

> By default, the vlayer node operates in DEV mode.
> Note that `PRODUCTION` mode is much slower than `DEV` mode. It is important to design protocols with proving execution time in mind, which takes at least a few minutes.
> DEV mode only works on development and test chains to avoid accidental mistakes.
