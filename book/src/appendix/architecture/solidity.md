# Solidity

## On-chain proving

On-chain verification is implemented by using a customized function with the following arguments:
- `Proof` structure, which contains data required to verify arguments from the point below
- list of arguments in the same order as returned by the prover (public output)
- optionally, user defined additional arguments

The verification function should use `onlyVerified()` modifier, which takes a single argument, the address of a smart contract that was executed off-chain.

Example verification function:

```solidity
contract Example is Verifier {

    function claim(Proof proof, address verifiedArg1, uint verifiedArg2, bytes extraArg) public returns (uint)
        onlyVerified(PROVER_ADDRESS) {
        //...
    }

}
```

## Data flow

Proving data flow is composed of three steps. It starts at guest, which returns `GuestOutput`. GuestOutput consist of two fields: `execution_commitment` and `evm_call_result`. See the code snippets below for pseudocode:

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

> Note that ExecutionCommitment structure is generated based on solidity code from `Steel::Commitment`, with `sol!` macro.

Then data is prepended on the Host with two additional fields `length` and `seal`. Host returns it via JSON RPC `v_call` method, as string of bytes, in `result` field.

Finally, the method on on-chain smart-contract is called. For that purpose it is prepended with function selector and might be additionally appended with custom user arguments.

The `Proof` structure looks as follows and benefits from the


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

`length` represents the length of journal data, which is located in `msg.data`, starting at byte 0 of `executionCommitment` and ends with last byte of last verified arg.

`onlyVerified` gets `seal` and Journal from `msg.data` and use them to verify computations.

Below is a schema of how block of data is encoded in different structures at different stages.

![Schema](/images/architecture/transaction-data.png)


> Thanks to clever slicing of data, there is no need for additional repackaging or recoding. The smart contracts, can be called with arguments, that can be easily manipulated, without extra deserialization process. JavaScript client which calls JSON RPC API, have easy access to the variables as well.
