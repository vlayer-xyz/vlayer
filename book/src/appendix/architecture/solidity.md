# Solidity

### On-chain proving

On-chain verification should be done using a user-defined function with the following arguments:
- `VProof proof`
- list of arguments in the same order as returned  
- user defined additional params

The verification function should then use `onlyVerified()` modifier, which takes `VProof` and `ProverOutput` from the calldata and does the actual proof verification.

```solidity
struct VProof {
    uint16 offset;
    uint32 length;
    uint16 version;
    uint32 chainId;
    uint128 blockNumber;
    bytes32 blockHash;
    bytes seal;
```
}

Therefore, user-defined verification functions can look like the following example:
```solidity

contract Airdrop is VlayerVerifier {

    function claim(VProof proof, address tokenAddress) public returns (uint) onlyVerified() {

    }

}

```