# Solidity

## Proving

On-chain verification is implemented by using a customized verification function. It receives a list of arguments in the same order as returned by the `Prover` (public output).

> `Proof` structure must always be returned from the `Prover` as the first returned element (more on that [here](/advanced/prover.html#proof)),
> which means that `Proof` structure must also be passed as the first argument to the verification function. 

The verification function should use the `onlyVerified()` modifier, which takes two arguments: the address of a smart contract and a selector of function that was executed in the `Prover` contract.

See an example verification function below:

```solidity
contract Example is Verifier {

    function claim(Proof _p, address verifiedArg1, uint verifiedArg2, bytes extraArg) public returns (uint)
        onlyVerified(PROVER_ADDRESS, FUNCTION_SELECTOR) {
        //...
    }

}
```

>`proof` is not an argument to `onlyVerified` because it is automatically extracted from `msg.data`.

## Data flow

Proving data flow consists of three steps:

### Step 1: GuestOutput

It starts at `Guest`, which returns `GuestOutput` structure. 
`GuestOutput` consists of just one field - `evm_call_result`. `evm_call_result` field is abi encoded `Prover` function output. 
Since `Prover` returns `Proof` placeholder as its first returned value, `Guest` pre-fills `length` and `call_assumptions` fields of the `Proof` structuture. 

`length` field of `Proof` structure is equal to the length of abi encoded _public outputs_, not including size of `Proof` placeholder.  

See the code snippets below for pseudocode:

```rust
pub struct GuestOutput {
    pub evm_call_result: Vec<u8>,
}
```

![Schema](/images/architecture/guest-output.png)


### Step 2: Host output as `v_call` result

In the next step, the `Host` replaces the `seal` field in the `Proof` placeholder with the actual `seal`, 
which is a cryptographic proof of the `Prover`'s execution. 
The `Host` then returns this via the JSON-RPC `v_call` method, delivering the `seal` as a byte string in the `result` field.

This approach allows the smart contract developer to decode the 
`v_call` result as though they were decoding the `Prover` function's output directly.
In other words, the `v_call` result is compatible with, and can be decoded according to, the `ABI` of the called `Prover` function.

### Step 3: Verifier call
Finally, the method on the on-chain smart contract is called to verify the proof. More on that in the next section. 

## Proof verification

To verify a zero-knowledge proof, vlayer uses a `verify` function, delivered by [Risc-0](https://dev.risczero.com/api/blockchain-integration/contracts/verifier).

```solidity
function verify(Seal calldata seal, bytes32 imageId, bytes32 journalDigest) { /* ... */ }
```

`onlyVerified` gets `seal` and `journalDigest` by slicing it out of `msg.data`. 

`length` field of `Proof` structure is used, when guest output bytes are restored in `Solidity` in order to compute `journalDigest`.
`length` field hints the verifier, which bytes should be included in the journal, since they belong to encoding of the _public outputs_, 
and which bytes belong to _[extra arguments](/advanced/verifier.html#extra-arguments)_, passed additionally in calldata. 

`imageId` is fixed on blockchain and updated on each new version of vlayer.

## Data encoding summary

Below, is a schema of how a block of data is encoded in different structures at different stages.

![Schema](/images/architecture/transaction-data.png)

## Structures
The `Proof` structure looks as follows:

```solidity
struct Proof {
    uint32 length;
    Seal seal;
    CallAssumptions callAssumptions;
}
```

with `Seal` having the following structure: 

```solidity
enum ProofMode {
    GROTH16,
    FAKE
}

struct Seal {
    bytes32[8] seal;
    ProofMode mode;
}
```

and the following structure of `CallAssumptions`:

```solidity
struct CallAssumptions {
    address proverContractAddress;
    bytes4 functionSelector;
    uint256 settleBlockNumber;
    bytes32 settleBlockHash;
}
```

> Note that `Proof`, `Seal` and `CallAssumptions` structures are generated based on Solidity code from  with `sol!` macro.


## Two Proving Modes

To support two [proving modes](/advanced/proving.html), vlayer provides a set of smart contracts connected to the `Verifier` contract, one for each mode:

- `DEVELOPMENT` - Automatically deployed with each `Prover` contract, but only on development and test networks. This mode will be used if the `ProofMode` decoded from `SEAL` is `FAKE`.
- `PRODUCTION` - This requires infrastructure deployed ahead of time that performs actual verification. This mode will be used if the `ProofMode` decoded from `SEAL` is `GROTH16`.
