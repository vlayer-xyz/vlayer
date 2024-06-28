# Teleport

## Ethereum ecosystem of chains
The Ethereum ecosystem is fragmented, consisting of various EVM chains such as Arbitrum, Optimism, Base, and many more. Developing applications that interact with multiple chains used to be challenging, but Teleport makes it easy.

## Teleporting betweens chains
`setChainId(uint chainId)` function, availble in Prover contracts, allows to switch the context of execution to another chain (teleport).  It takes a single argument `chainId`, which specifies the chain in the context of which the next function call will be executed.

## Example 

The example below ilustrates checking NFT balances on two different chains:

```solidity
contract NftOwnership is Prover {

  function check_byac_ownership()  {
    require(
      IERC721(BYAC_NFT_ADDR).balanceOf(msg.sender) > 0, "not owning any BYAC"
    );
  }

  function check_sandbox_ownership() {
    require(
      IERC721(SANDBOX_NFT_ADDR).balanceOf(msg.sender) > 0, "not owning any Sandbox"
    );
  }

  function main() public {
    setChainId(1);  // next function call will be teleported to Ethereum
    check_byac_ownership() // checks balanceOf at Ethereum

    setChainId(42161); // next function call will be teleported to Arbitrum
    check_sandbox_ownership() // checks balanceOf at Arbitrum
  }
}
```

