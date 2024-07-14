# Prover

vlayer node is a http server that serves as a prover. You can run it with following command:

```sh
vlayer serve
```

By default, it accepts http requests on port `3000` in the format specified in [JSON-RPC API appendix](/appendix/api.md)


## Proving modes

vlayer node provides two proving modes:
- `DEV` - intended to be used for development and testing only. It only executes code and verify correctness of execution. It doesn't perform actual proving. In this mode the Verifier contract will check correctness of computations, but it can be cheated by malicious prover.
- `PRODUCTION` - intended to be used on production and final testing. It does performs actual proving.

> By default, vlayer node works in DEV mode.

> Note that `PRODUCTION` mode  is much slower then `DEV` mode. It is important, to design protocols with proving execution time in mind, which takes at least a few minutes.

> DEV mode only work in on development and test chains to avoid accidental mistake.
