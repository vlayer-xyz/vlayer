# FAQ & Troubleshooting

### My mainnet gas balance is below 1M, but enough for a proof. Why am I getting `error: Allocating gas failed with error: Allocating gas: JSON-RPC error: {"code":1003,"message":"Insufficient gas balance"}`

vlayer zkEVM prover pre-allocates gas before running a proof. It does this using the `gasLimit` parameter passed to the `vlayerClient.prove()` call.
If `gasLimit` is higher than your total balance, allocation fails — even if the actual proof would use less gas.

By default, `gasLimit` is set to `1_000_000`.

If you're using one of our example templates, `GAS_LIMIT` is an environment variable that overrides this default.

To fix the error, one can reduce `gasLimit` so it's below your actual balance, but still high enough for the proof to succeed.