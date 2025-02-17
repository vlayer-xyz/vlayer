# Time Travel and Teleport

vlayer allows seamless aggregation of data from different blocks and chains. We refer to these capabilities as Time Travel and Teleport. How is it done?

> **Note:** Teleportation is currently possible only from L1 chains to L2 optimistic chains. We plan to support teleportation from L2 to L1 in the future.

## Verification

At the [beggining of the `guest::main`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/guest/src/guest.rs#L38) we verify whether the data for each execution location is coherent. However, we have not yet checked whether data from multiple execution locations align with each other. Specifically, we need to ensure that:

- The blocks we claim to be on the same chain are actually there (allowing time travel between blocks on the same chain).
- The blocks associated with a given chain truly belong to that chain (enabling teleportation to the specified chain).

The points above are verified by the [`Verifier::verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/travel_call.rs#L80) function. The `Verifier` struct is used both during the host preflight and guest execution. Because of that it is parametrized by Recording Clients (in host) and Reading Clients (in guest).

The `verify` function performs above verifications by:

### I. Time Travel Verification

Is possible thanks to [Chain Proofs](./chain_proof.md).
Verification steps are as follows:

1. **Retrieve Blocks:** Extract the list of blocks to be verified and group them by chain.
2. **Iterate Over Chains:** For each chain run [time travel `verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/time_travel.rs#L40) function on its blocks.
3. **Skip Single-Block Cases:** If only one block exists, no verification is needed.
4. **Request Chain Proof:** Fetch cryptographic proof of chain integrity.
5. **Verifies Chain Proof:** Run the [chain proof `verify`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/chain/common/src/verifier.rs#L46) function on the obtained Chain Proof to check its validity.
6. **Validate Blocks:** Compare each block’s hash with the hash obtained by block number from the validated Chain Proof.

### II. Teleport Verification

1. **Identify Destination Chains:** Extract execution locations from `CachedEvmEnv`, filtering for chains different from the starting one.
2. **Skip Local Testnets:** If the source chain is a local testnet, teleport verification is skipped.
3. **Validate Chain Anchors:** Ensure the destination chain is properly anchored to the source chain using [`assert_anchor()`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/chain/src/optimism.rs#L25).
4. **Fetch Latest Confirmed L2 Block:** Use the [`AnchorStateRegistry`](https://docs.optimism.io/stack/smart-contracts#anchorstateregistry) and `sequencer_client` to get the latest confirmed block on the destination chain.
5. **Verify Latest Confirmed Block Hash Consistency:** Compare the latest confirmed block’s hashes.
6. **Verify Latest Teleport Location Is Confirmed:** Using function [`ensure_latest_teleport_location_is_confirmed`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/verifier/teleport.rs#L154) we check that latest destination block number is not greater than latest confirmed block number.

## Verifier Safety & Testability

To prevent unauthorized custom verifier implementations, we use [Sealed trait pattern](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/). This ensures that [`IVerifier`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/common/src/verifier/sealing.rs) trait cannot be implemented outside the file it was defined - except when the `testing` feature is enabled.

This design is crucial because verifiers are composable. When testing a `Verifier` that is composed from other verifiers, we need to mock them with fake implementations. This flexibility is achieved by allowing special implementations under the `testing` feature.

### Macros Overview

The following macros work together to enforce sealing and enable test mocking:

- **`sealed_trait!`** - Creates a private module (`seal`) containing a trait `Sealed`. By requiring verifier traits to extend `seal::Sealed`, only types that also implement Sealed (and hence are defined within controlled environment) can implement the verifier traits.
- **`verifier_trait!`** - Defines the actual verifier trait (e.g., `IVerifier`) with a verify method. The trait extends `seal::Sealed`.
- **`impl_verifier_for_fn!`** - Allows functions to be used as verifiers by implementing the verifier trait for them. This is only enabled in testing (or when the `testing` feature is turned on).
- **`impl_sealed_for_fn!`** - Implements the `Sealed` trait for functions with the appropriate signature.
- **`sealed_with_test_mock!`** - This is a convenience macro that ties everything together. It:
  - Calls `sealed_trait!` to create the `Sealed` trait
  - Calls `impl_sealed_for_fn!` to allow function pointers to be sealed
  - Defines verifier trait using `verifier_trait!`
  - Implements the verifier trait for function pointers with `impl_verifier_for_fn!`

## Inspector

Both **Time Travel** and **Teleport** features are made possible by the [`Inspector`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call/inspector.rs#L26) struct, a custom implementation of the `Inspector` trait from REVM. Its purpose is to handle **travel calls** that alter the execution context by switching the blockchain network or block number.

How does it work? When `ExecutionLocation` is updated, `Inspector`:

1. Creates a separate EVM with new `ExecutionLocation` context (using `transaction_callback` function passed as argument).
2. Executes the subcall on a separate inner EVM with updated location.

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

- `set_block(block_number)`: Updates the block number while keeping the same chain.
- `set_chain(chain_id, block_number)`: Changes both the blockchain network and block number.

#### 3. Intercepts Contract Calls

Intercepts every contract call and determines how to handle it:

- **Precompiled Contracts:** If the call targets a precompiled contract, it logs the call and records relevant metadata.
- **Travel Call Contract:** If the call is directed to the designated travel call contract (identified by `CONTRACT_ADDR`), the `Inspector` parses the input arguments and triggers a travel call by invoking either `set_block` or `set_chain`.
- **Standard Calls:** If no travel call is detected, the `Inspector` allows the call to proceed normally. However, if a travel call has already set a new context, it is processed using the provided `transaction_callback` and applies the updated execution context in the [`on_call`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call/inspector.rs#L68) function.

#### 4. Monitors & Logs Precompiled Contracts

If the call is made to a precompiled contract it logs the call and records metadata.

Precompiles used by vlayer are listed [here](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/precompiles/src/lib.rs#L24).

### `ExecutionResult` to `CallOutcome` conversion

[`ExecutionResult`](https://github.com/bluealloy/revm/blob/dd63090f2a8663714778e0224df3602cb0f8928f/crates/context/interface/src/result.rs#L40) and [`CallOutcome`](https://github.com/bluealloy/revm/blob/main/crates/interpreter/src/interpreter_action/call_outcome.rs#L16) are revm structs used in the `Inspector` code. They are necessary to make travel calls work.

- `ExecutionResult` is an enum representing the complete outcome of a **transaction**. It has three variants—`Success`, `Revert`, and `Halt`—and includes transaction information such as gas usage, gas refunds, logs, and output data.
- `CallOutcome` is a struct representing the result of a single **call** within the EVM interpreter. It encapsulates an [`InterpreterResult`](https://github.com/bluealloy/revm/blob/25d9726522f8f88373ba2105a97adbd509e81683/crates/interpreter/src/interpreter.rs#L170) (which contains output data and gas usage) along with a `memory_offset` (the range in memory where the output data is located).

Most fields stored in `ExecutionResult` have equivalents in `CallOutcome`. The only exceptions are`logs` and `gas_refunded` fields from `ExecutionResult::Success`, which do not exist in `CallOutcome`. Conversely, `CallOutcome` includes `memory_offset`, which has no direct counterpart in `ExecutionResult`.

When `Inspector::call` is executed, it must return a `CallOutcome`. However, the `transaction_callback` run inside `Inspector::call` executes the full EVM and returns an `ExecutionResult`. Hence, the conversion between the two is needed.

This conversion is performed using the [`execution_result_to_call_outcome`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/utils/evm_call.rs#L21) function [within `Inspector::on_call`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call/inspector.rs#L80). During this process `logs` and `gas_refunded` fields from `ExecutionResult::Success` are discarded, as they are not required in `CallOutcome`. `memory_offset` is obtained from `CallInputs`, which is also passed to `execution_result_to_call_outcome` as an argument.

## Executor

[`Executor`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call.rs#L28) struct handles running EVM transactions. `Inspector` is created by the `Executor` struct and used while [building EVM](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call/evm.rs#L12).

```rust
pub struct Executor<'envs, D: RevmDB> {
    envs: &'envs CachedEvmEnv<D>,
}
```

### `call`

The `Executor` provides a public [`call`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call.rs#L33) method that runs the internal execution ([`internal_call`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call.rs#L41)).

### `internal_call`

The private [`internal_call`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call.rs#L41) method performs the core execution of an EVM transaction, including support for recursive internal calls (when one smart contract calls another). In this implementation, the `envs` are shared across recursive calls, meaning that any modification performed by one call is visible to others.

But updates to the database `state` (contained in the [`ProofDb`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/host/src/db/proof.rs#L26) structure, being a part of `env`) are safe because the `state` is modified only by inserting new entries. New keys are added to the `accounts`, `contracts`, and `block_hash_numbers` collections, while existing entries remain unchanged.

#### Error handling

Due to the design of revm's `Inspector` trait, the `Inspector::call` (run inside EVM build in `Executor::internal_call`) method must return an `Option<CallOutcome>` rather than a `Result`. This limitation means that errors occurring during intercepted calls cannot be directly propagated via the return type.

To work around this constraint, our `Inspector` implementation [uses panics to signal errors](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call/inspector.rs#L77). The panic is then [caught in the `Executor::call`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/travel_call.rs#L36) method using `panic::catch_unwind`. This mechanism allows us to convert panics into proper error results, ensuring that errors are not lost, even though the `Inspector::call` function itself cannot return an error.

## On-chain Verification

When the proving process begins, a specific block is selected as the **settlement block**—the block we commit to. Then, a call to the `Prover` contract is executed within zkEVM environment. The guest proof is valid **providing** the block and contract assumptions used during its generation are accurate.

These assumptions are encapsulated in a dedicated struct used within the guest code:

```solidity
struct CallAssumptions {
    address proverContractAddress;
    bytes4 functionSelector;
    uint256 settleBlockNumber;
    bytes32 settleBlockHash;
}
```

The struct is [created](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/guest/src/guest.rs#L59) inside the `guest::main` function. Since the guest itself **cannot independently prove the validity of these assumptions**, they must be verified externally.

To achieve this, `CallAssumptions` is included in the [`GuestOutput`](https://github.com/vlayer-xyz/vlayer/blob/main/rust/services/call/engine/src/io.rs#L107) and subsequently verified **on-chain** using the `Verifier` contract, specifically through the [`_verifyExecutionEnv`](https://github.com/vlayer-xyz/vlayer/blob/main/contracts/vlayer/src/proof_verifier/ProofVerifierBase.sol#L42). This verification ensures that the proof aligns with a valid blockchain state.

### Validation Steps in `_verifyExecutionEnv`

The `_verifyExecutionEnv` function checks the following:

1. **Prover Contract Validation**: Ensures that the proof comes from the correct `proverContractAddress`.
2. **Function Selector Validation**: Verifies that the function being executed matches the expected function selector.
3. **Block Number Validation**: Ensures that the proof is based on a **past block** (not from the future) and that the block falls within the **last 256 blocks**—the maximum number of historical blocks accessible during EVM execution.
4. **Block Hash Validation**: Confirms that the `settleBlockHash` matches the actual on-chain block hash at the `settleBlockNumber`.
