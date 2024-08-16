# Solidity

## Proving

On-chain verification is implemented by using a customized function with the following arguments:
- the `Proof` structure, which contains data required to verify arguments from the point below
- a list of arguments in the same order as returned by the `Prover` (public output)
- optionally, user defined additional arguments

The verification function should use the `onlyVerified()` modifier, which takes two arguments, the address of a smart contract and a selector of function that was executed in the `Prover` contract.

See example verification function below:

```solidity
contract Example is Verifier {

    function claim(Proof _p, address verifiedArg1, uint verifiedArg2, bytes extraArg) public returns (uint)
        onlyVerified(PROVER_ADDRESS, FUNCTION_SELECTOR) {
        //...
    }

}
```

> `proof` is not an argument to `onlyVerified` because it is automatically extracted from `msg.data`.

### Special variables
There are special Solidity variables and functions that exist in the global namespace. These entities are mainly used to provide information about the current state of the blockchain in which the code is executed, or are general utility functions. 

Since prover contracts are executed in the [vlayer zkEVM environment](/appendix/architecture/prover.html), some variables are not implemented or behave in a different way than in regular EVM chains. Below is a list of all variables available to vlayer provers with their description.

Variables that have special implemention in vlayer zkEVM:
* `block.number(uint)`: current block number, controlled either by vlayer prover [JSON-RPC call](/appendix/api.html) or [teleport](/features/teleport.html) / [time-travel](/features/time-travel.html) functions
* `msg.sender`: cannot be used as in typical Solidity as it always points to the same address
* `block.chainid(uint)`: current chain id, controlled either by vlayer prover [JSON-RPC call](/appendix/api.html) or [teleport](/features/teleport.html) / [time-travel](/features/time-travel.html) functions

Not available variables: 
* `block.basefee(uint)`: returns 0
* `block.blobbasefee(uint)`: returns 0
* `block.coinbase(address payable)`: returns 0x0 zero address
* `block.difficulty(uint)`: returns 0
* `block.gaslimit (uint)`: returns 0
* `block.prevrandao(uint)`: returns 0
* `msg.value`: payable functionalities are not supported, returns 0

Same behaviour as in regular Solidity:
* `blockhash(uint blockNumber)`: hash of the given block when blocknumber is one of the 256 most recent blocks; otherwise returns zero
* `blobhash(uint index)`: versioned hash of the index-th blob associated with the current transaction. Returns zero if no blob with the given index exists.
* `block.timestamp (uint)`: current block timestamp as seconds since unix epoch
* `gasleft()`: remaining gas
* `msg.data`: complete calldata
* `msg.sig`: first four bytes of the calldata (i.e. function identifier)
* `tx.gasprice`: gas price of the transaction
* `tx.origin`: sender of the transaction (full call chain)

> Note that since Verifier contracts can be deployed on various chains, they may support a different set of special variables specifc to given chain. Always check official docs for reference.

## Data flow

Proving data flow is composed of three steps. It starts at `Guest`, which returns `GuestOutput`. `GuestOutput` consist of two fields: `execution_commitment` and `evm_call_result`.

See the code snippets below for pseudocode:

```rust
pub struct GuestOutput {
    pub execution_commitment: ExecutionCommitment,
    pub evm_call_result: Vec<u8>,
}
```

```solidity
struct ExecutionCommitment {
    address proverContractAddress;
    bytes4 functionSelector;
    uint256 settleBlockNumber;
    bytes32 settleBlockHash;
```

> Note that `ExecutionCommitment` structure is generated based on Solidity code from `Vlayer::Commitment`, with `sol!` macro.

Then the data is prepended on the `Host` with two additional fields `length` and `seal`. The `Host` returns it via JSON-RPC `v_call` method, as a string of bytes, in the `result` field.

Finally, the method on the on-chain smart contract is called. For that purpose, it is prepended with a function selector and might be additionally appended with custom user arguments.

The `Proof` structure looks as follows:


```solidity
struct Proof {
    uint32 length;
    bytes seal;

    ExecutionCommitment executionCommitment;
}
```

## Zero-knowledge proof verification

To verify a zero-knowledge proof, vlayer uses a `verify` function, delivered by Risc-0.

```solidity
function verify(bytes calldata seal, bytes32 imageId, bytes32 journalDigest)
```

`Proof.length` represents the length of journal data, which is located in `msg.data`, starting at byte 0 of `executionCommitment` and ends with the last byte of the last verified argument.

`onlyVerified` gets `seal` and `journalDigest` by slicing it out of `msg.data`. This is where `length` is used.

`imageId` is fixed on blockchain and updated on each new version of vlayer.

## Data encoding summary

Below, is a schema of how a block of data is encoded in different structures at different stages.

![Schema](/images/architecture/transaction-data.png)


> Thanks to clever slicing of data, there is no need for additional repackaging or recoding. The smart contracts, can be called with arguments, that can be easily manipulated, without extra deserialization process. JavaScript client which calls JSON-RPC API, have easy access to the variables as well.

## Two Proving Modes

To support two [proving modes](/advanced/proving.md), vlayer provides a set of smart contracts connected to the `Verifier` contract, one for each mode:

- `DEVELOPMENT` - Automatically deployed with each `Prover` contract, but only on development and test networks. This mode will be used if the `ProofMode` decoded from `SEAL` is `FAKE`.
- `PRODUCTION` - This requires infrastructure deployed ahead of time that performs actual verification. This mode will be used if the `ProofMode` decoded from `SEAL` is `GROTH16`.
