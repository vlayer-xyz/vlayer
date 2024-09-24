# Proving Canonicity

It is essential to be able to verify the latest block hash on-chain.

Without that - an attacker would be able to:
* Execute code on some made-up chain with made-up data
* Execute code on some non-canonical fork

## blockhash

Solidity/EVM has a built-in function that allows us to do that.

```sol
blockhash(uint blockNumber) returns (bytes32)
```
It returns a hash of the given block when `blockNumber` is one of the **256** most recent blocks; otherwise returns zero.

We assert result of this function with the block hash found in the execution commitment of the call proof.

### blockhash limitations
However, this method is limited, as it only works for the most recent 256 blocks on a given chain.

256 blocks is not a measure of time. We need to multiply it by block time to know - how much time we have to settle the proof on a specific chain.


* **Ethereum**: 12 seconds - 51 minutes
* **Optimism**: 2 seconds - 8.5 minutes
* **Arbitrum One**: 250ms - 1 minute

With current prover performance - it takes 2 minutes to generate a no-op proof so we need a solution.

### Block Pinning

Instead of waiting for the proof - we can have a smart-contract that **pins** block hashes we are planning to use in storage.

Therefore, the flow will look like this:
* As soon as Host is ready to generate the proof - it will do two things in parallel:
    * Send a transaction on-chain pinning the latest block
    * Start generating the proof
* When proof is ready, and we settle on-chain we:
    * First try to use `blockhash`
    * If it fails - fallback to the list of pinned blocks

This is not implemented yet.
