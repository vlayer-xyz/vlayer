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
1. **Retrieve Blocks:** Extract the list of blocks to be verified and group them by chain.
2. **Iterate Over Chains:** For each chain run [time travel `verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/time_travel.rs#L40) function on its blocks that does the following:
3. **Skip Single-Block Cases:** If only one block exists, no verification is needed.
4. **Request Chain Proof:** Fetch cryptographic proof of chain integrity.
5. **Verifies Chain Proof:** Run the [chain proof `verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/chain/common/src/verifier.rs#L46) function on the obtained Chain Proof to check its validity.
6. **Validate Blocks:** Compare each block’s hash with the hash obtained by block number from the validated Chain Proof.

<!-- potentially todo: document chain proof `verify` function -->

### II. Teleport Verification
1. **Identify Destination Chains:** Extract execution locations from `CachedEvmEnv`, filtering for chains different from the starting one.
2. **Skip Local Testnets:** If the source chain is a local testnet, teleport verification is skipped.
3. **Validate Chain Anchors:** Ensure the destination chain is properly anchored to the source chain using [`assert_anchor()`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/chain/src/optimism.rs#L25).
4. **Fetch Latest Confirmed L2 Block:** Use the [`AnchorStateRegistry`](https://docs.optimism.io/stack/smart-contracts#anchorstateregistry) and `sequencer_client` to get the latest confirmed block on the destination chain.
5. **Verify Latest Confirmed Block Hash Consistency:** Compare the latest confirmed block’s hashes.
6. **Verify Latest Teleport Location Is Confirmed:** Using function [`ensure_latest_teleport_location_is_confirmed`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/teleport.rs#L154) we check that latest destination block number is not greater than latest confirmed block number.

<!-- potentially todo: document how we are using AnchorStateRegistry in more detail -->

<!-- todo: picture -->

## Inspector

After verifying that execution locations belong to their respective chains, we can perform travel calls on them. How is this achieved?

Both **Time Travel** and **Teleport** are enabled by the `Inspector` struct, a custom implementation of the `Inspector` trait from REVM. Its purpose is to **intercept**, **monitor**, and *modify* EVM calls, particularly handling **travel calls** that alter the execution context by switching the blockchain network or block number.

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
Intercepts every contract call and determines how to handle it:
* **Precompiled Contracts:** If the call targets a precompiled contract, it logs the call and records relevant metadata.
* **Travel Call Contract:** If the call is directed to the designated travel call contract (identified by `CONTRACT_ADDR`), the `Inspector` parses the input arguments and triggers a travel call by invoking either `set_block` or `set_chain`.
* **Standard Calls:** If no travel call is detected (i.e., no execution context update has occurred), the `Inspector` allows the call to proceed normally. However, if a travel call has already set a new context, it processes the call using the provided `transaction_callback` and applies the updated execution context.

#### 4. Monitors & Logs Precompiled Contracts
If the call is made to a precompiled contract it logs the call and records metadata.

<!-- todo: write about precompiles -->