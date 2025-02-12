# Time Travel and Teleport

vlayer allows seamless aggregation of data from different blocks and chains. We refer to these capabilities as Time Travel and Teleport. How is it done?

> **Note:** Teleportation is currently possible only from L1 chains to L2 optimistic chains. We plan to support teleportation from L2 to L1 in the future.

## Verification

At the [beggining of the `guest::main`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/guest/src/guest.rs#L38) we verify whether the data for each execution location is coherent. However, we have not yet checked whether data from multiple execution locations align with each other. Specifically, we need to ensure that:
* The blocks we claim to be on the same chain are actually there (allowing time travel between blocks on the same chain).
* The blocks associated with a given chain truly belong to that chain (enabling teleportation to the specified chain).
The points above are verified by the [`Verifier::verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/travel_call.rs#L80) function. The `Verifier` struct is used both during the host preflight and guest execution. Because of that it is parametrized by Recording Clients (in host) and Reading Clients (in guest).

The `verify` function performs above verifications by:

### I. Time Travel Verification
Is possible thanks to [Chain Proofs](./chain_proof.md).
Verification steps are as follows:
1. **Retrieve Blocks:** Extracts the list of blocks to be verified and group them by chain.
2. **Iterate Over Chains:** For each chain runs [time travel `verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/time_travel.rs#L40) function on its blocks.
3. **Skip Single-Block Cases:** If only one block exists, no verification is needed.
4. **Request Chain Proof:** Fetches cryptographic proof of chain validity.
5. **Verifies Chain Proof:** Runs the [chain proof `verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/chain/common/src/verifier.rs#L46) function to check if the Chain Proof is valid.
6. **Validate Blocks:** Compares each block’s hash with the hash obtained from the validated Chain Proof by block number.

### II. Teleport Verification
1. **Identify Destination Chains:** Extracts execution locations from `CachedEvmEnv`, filtering for chains different from the starting one.
2. **Skip Local Testnets:** If the source chain is a local testnet, teleport verification is skipped.
3. **Validate Chain Anchors:** Ensures the destination chain is properly anchored to the source chain using [`assert_anchor()`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/chain/src/optimism.rs#L25).
4. **Fetch Latest Confirmed L2 Block:** Uses the [`AnchorStateRegistry`](https://docs.optimism.io/stack/smart-contracts#anchorstateregistry) and `sequencer_client` to get the latest confirmed block on the destination chain.
5. **Verify Block Hash Consistency:** Compares the latest confirmed block’s hash with the execution environment's expected state.

## Inspector

Both Time Travel and Teleport are made possible by the `Inspector` struct, a custom implementation of the `Inspector` trait from REVM. Its purpose is to intercept, monitor, and modify EVM calls, particularly handling "travel calls" that alter the execution context by switching the blockchain network or block number.

```rust
pub struct Inspector<'a> {
    start_chain_id: ChainId,
    pub location: Option<ExecutionLocation>,
    transaction_callback: Box<TransactionCallback<'a>>,
    metadata: Vec<Metadata>,
}
```

### Key Responsibilities of the Inspector

#### 1. Tracks Execution Context (Chain & Block Info)
It maintains the `ExecutionLocation` which consists of `chain_id` and `block_number`

#### 2. Handles Travel Calls
There are two special functions that modify execution context:
* `set_block(block_number)`: Updates the block number while keeping the same chain.
* `set_chain(chain_id, block_number)`: Changes both the blockchain network and block number.

#### 3. Intercepts Contract Calls
When a call is made, the Inspector determines if it should:
* Process a travel call (`set_block` or `set_chain`),
* Forward the call to a custom transaction handler (`transaction_callback`),
* Continue normal execution 

#### 4. Monitors & Logs Precompiled Contracts
If the call is made to a precompiled contract it logs the call and records metadata.

