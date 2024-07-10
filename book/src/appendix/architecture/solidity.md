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

    function claim(Proof proof, address verifiedArg1, uint verifiedArg2, bytes extraArg) public returns (uint)
        onlyVerified(PROVER_ADDRESS, FUNCTION_SIGNATURE) {
        //...
    }

}
```

> `proof` is not an argument to `onlyVerified` because it is automatically extracted from `msg.data`.

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
    address startContractAddress;
    bytes4 functionSelector;
    uint256 settleBlockNumber;
    bytes32 settleBlockHash;
}
```

> Note that `ExecutionCommitment` structure is generated based on Solidity code from `Steel::Commitment`, with `sol!` macro.

Then the data is prepended on the `Host` with two additional fields `length` and `seal`. The `Host` returns it via JSON-RPC `v_call` method, as a string of bytes, in the `result` field.

Finally, the method on the on-chain smart contract is called. For that purpose, it is prepended with a function selector and might be additionally appended with custom user arguments.

The `Proof` structure looks as follows and benefits from the:


```solidity
struct Proof {
    uint32 length;
    bytes seal;

    ExecutionCommitment executionCommitment;
    //uint256 startBlockNo; (future versions)
    //uint256 startChainId (future versions)
    // startContractAddress: Address;
    // functionSelector: bytes4;
    // uint256 settleBlockNumber;
    // bytes32 settleBlockHash;
}
```

## Zero-knowledge proof verification

To verify a zero-knowledge proof, vlayer uses a `verify` function, delivered by Risc-0.

```solidity
function verify(bytes calldata seal, bytes32 imageId, bytes32 journalDigest)
```

`length` represents the length of journal data, which is located in `msg.data`, starting at byte 0 of `executionCommitment` and ends with the last byte of the last verified argument.

`onlyVerified` gets `seal` and `journalDigest` by slicing it out of `msg.data`. This is where `length` is used. 

`imageId` is fixed on blockchain and updated on each new version of vlayer.

## Summary

Below is a schema of how block of data is encoded in different structures at different stages.

![Schema](/images/architecture/transaction-data.png)


> Thanks to clever slicing of data, there is no need for additional repackaging or recoding. The smart contracts, can be called with arguments, that can be easily manipulated, without extra deserialization process. JavaScript client which calls JSON-RPC API, have easy access to the variables as well.
