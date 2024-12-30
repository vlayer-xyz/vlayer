# Proving Canonicity

It is essential to be able to verify the canonicity of the latest block hash on-chain.

Without that - an attacker would be able to successfully submit the proof generated on:
* a made-up chain with prepared, malicious data;
* a non-canonical fork.

## blockhash

Solidity/EVM has a built-in function that allows us to do that.

```sol
blockhash(uint blockNumber) returns (bytes32)
```
It returns a hash of the given block when `blockNumber` is one of the **256** most recent blocks; otherwise returns zero.

We assert result of this function with the block hash found in the call assumptions of the call proof.

### blockhash limitations
However, this method is limited, as it only works for the most recent 256 blocks on a given chain.

256 blocks is not a measure of time. We need to multiply it by block time to know - how much time we have to settle the proof on a specific chain.


* **Ethereum**: 12 seconds - 51 minutes
* **Optimism**: 2 seconds - 8.5 minutes
* **Arbitrum One**: 250ms - 1 minute

With current prover performance - it takes a couple of minutes to generate a proof. That means by the time it's ready, we will already have missed the slot to settle on Arbitrum.

### Block Pinning

Instead of waiting for the proof - we can have a smart-contract that **pins** block hashes we are planning to use in storage.

Therefore, the flow will be like this:
* As soon as Host is ready to start the proof generation - it will do two things in parallel:
    * Send a transaction on-chain pinning the latest block
    * Start generating the proof
    
* Once the proof is ready, in order to settle on-chain we:
    * First try to use `blockhash`
    * If it fails - fallback to the list of pinned blocks

This is not implemented yet.

### EIP2935

[EIP2935](https://eips.ethereum.org/EIPS/eip-2935) proposes a very similar solution but on a protocol level.
Instead of pinning blocks - it requires nodes to make some (8192) range of blocks available through the storage of system contract.
It's planned to be included in a Pectra hard fork and go live on mainnet early 2025.