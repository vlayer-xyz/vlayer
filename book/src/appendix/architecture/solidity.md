# Solidity

### On-chain proving

On-chain verification is implemented by using a customized function with the following arguments
- `Proof proof`
- list of arguments in the same order as returned by the prover
- user defined additional params

The verification function should then use `onlyVerified()` modifier, which takes `Proof` and user arguments from the calldata and does the actual proof verification.

```solidity
struct Proof {
    uint32 length;
    uint32 version;
    uint32 startChainId;
    uint32 proofBlockNumber;
    bytes32 proofBlockHash;
    bytes seal;
}
```

Proof contains public input - metadata required to verify a proof, which are not intended to be used by developers.

> A clever trick is used to simplify syntax. First field of `Proof` is length, which designates the length of proof data. Therefore proof data can be reconstructed from `calldata` and consist of part of `Proof` structure and some subset of function arguments.

As a result, a customized verification function can look like the following example:

```solidity
contract Airdrop is Verifier {

    function claim(Proof proof, address resultArg1, uint resultArg2, bytes userData) public returns (uint) onlyVerified() {
        //...
    }

}
```