# Verifier contract
<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. If you experience any bugs, please let us know <a href="https://discord.gg/JS6whdessP" target="_blank">on our Discord</a>. We appreciate your patience. </p>
</div>

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