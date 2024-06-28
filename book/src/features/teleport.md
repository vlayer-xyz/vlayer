# Teleport
The Ethereum ecosystem is fragmented. There are EVM chains like Arbitrum, Optimism, zkSync, Aleph Zero, Base and many more. Building applications that use multiple chains can be difficult. However, we believe that cross-chain capabilities are worth exploring. 

That's where the **Teleport** feature comes in. We introduced the `setChainId(uint chainId)` function, which allows you to switch the next call context to another chain:

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
    setChainId(1);  // next function call is teleported
    check_byac_ownership() // checks balanceOf at Ethereum

    setChainId(42161); // next function call is teleported
    check_sandbox_ownership() // checks balanceOf at Arbitrum
  }
}
```