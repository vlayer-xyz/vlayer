# Time Travel and Teleport

Vlayer allows seamless aggregation of data different various blocks and chains. We refer to these capabilities as Time Travel and Teleport. How it is done?

## Inspector

Time Travel and Teleport are made possible by the `Inspector` struct, a custom implementation of the `Inspector` trait from REVM. Its purpose is to intercept, monitor, and modify EVM calls, particularly handling "travel calls" that alter the execution context by switching the blockchain network or block number.

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
It maintains the execution location (`ExecutionLocation`), which consists of `chain_id` and `block_number`

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

