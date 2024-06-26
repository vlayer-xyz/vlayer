# Teleport
As you may know, blockchain ecosystem is fragmented. There are EVM chains like: Arbitrum, Optimism, Base and many more. Building applications that use multiple chains is tricky.

That's where Teleport functionality helps. It allows proving stuff from various chains:

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