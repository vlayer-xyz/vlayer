# Teleport
As you may know, the blockchain ecosystem is fragmented. There are EVM chains such as: Arbitrum, Optimism, Base and many more. Building applications that use multiple chains is difficult.

That's where the **Teleport** feature helps. It allows to prove things from different chains:

```solidity
contract NftOwnership is VlayerProver {

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
    setChainId(1);  // next function call is teleported to Ethereum
    check_byac_ownership()

    setChainId(42161); // next function call is teleported to Arbitrum
    check_sandbox_ownership() 
  }
}
```