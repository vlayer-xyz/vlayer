# Verifier contract

vlayer provides `Verifier` smart contracts that allow on-chain verification of computations performed by `Prover` contracts. To use the output computed by `Prover` contract, follow the rules covered in the next section.

## Proof Verification 
Proof verification can be done by any function that uses the `onlyVerified` modifier and passes arguments in a particular way. We call such a function *verification function*. See the example below, with *verification function* `claim`.

```solidity
contract Example is Verifier {

    function claim(
      Proof _p, 
      address verifiedArg1, 
      uint verifiedArg2, 
      bytes extraArg
    ) 
      public 
      returns (uint)
      onlyVerified(PROVER_ADDRESS, FUNCTION_SELECTOR) 
    {
        //...
    }
}
```

### onlyVerified modifier
The `onlyVerified` modifier takes two arguments:
1. `Prover` contract address 
2. the signature of the `Prover` function used to generate the proof

Under the hood, `onlyVerified` reads the submitted Proof and public inputs from `msg.data`. It ensures that:
- The proof is valid and passes verification
- The public inputs match values returned from `Prover`
- The proof was created by the specified `Prover` contract and the specified function
This prevents invalid, tampered, or mismatched proofs from being accepted by the contract.

### Proof argument
Passing `Proof` as the first argument to the *verification function* is mandatory. Note that even though the proof is not used directly in the body of the verified function, `onlyVerified` will have access to it via `msg.data`.

### Verified arguments
After the proof, we need to pass verified arguments. Verified arguments are the values returned by the `Prover` contract function. We need to pass all the arguments returned by prover, in the same order and each of the same type.

See the example below.

```solidity
contract Prover {

  function p() return (Proof p, address verifiedArg1, uint256 verifiedArg2, bytes32 verifiedArg3) {
    ...
  }
}

contract Verifier {
  function v(Proof _p, address verifiedArg1, uint256 verifiedArg2, bytes32 verifiedArg3) 
}

```

> Note: Passing different variables (in terms of type, name, or order) would either revert execution or cause undefined behavior and should be avoided for security reasons.


### Extra arguments
Extra arguments can be passed to `Verifier` by using additional function. This function manages all additional operations connected with extra arguments and then calls the actual verification function.  

See the example below:

```solidity
function f(Proof _p, verifiedArg1, verifiedArg2, extraArg1, extraArg2) {
  ...
  v(_p, verifiedArg1, verifiedArg2);
}
```