# Teleport

## Ethereum ecosystem of chains
The Ethereum ecosystem is fragmented, consisting of various EVM chains such as Arbitrum, Optimism, Base, and many more. Developing applications that interact with multiple chains used to be challenging, but Teleport makes it easy.

## Teleporting betweens chains
`setChainId(uint chainId, uint blockNo)` function, available in Prover contracts, allows to switch the context of execution to another chain (teleport).  It takes two arguments:
* `chainId`, which specifies the chain in the context of which the next function call will be executed
* `blockNo`, which is the block number of the given chain

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
    setChainId(1, 12_000_000);  
    // function call below will be teleported to Ethereum, block 12,000,000
    check_byac_ownership() // checks balanceOf at Ethereum

    setChainId(42161, 9_000_000); 
    // function call below will be teleported to Arbitrum, block 9,000,000
    check_sandbox_ownership() // checks balanceOf at Arbitrum
  }
}
```

First, call to `setChainId(1, 12_000_000)` configures the desired chain to the Ethereum mainnet (`chainId = 1`). Then, `check_byac_ownership` function ensures `msg.sender` owns of one of Bored Ape Yacht Club NFT on the Ethereum Mainnet at `12,000,000` block. In case caller doesn't have balance of specified NFT, contract would abort execution and throw error (thanks to `require()` check).

Next call `setChainId(42161, 9_000_000)` switches the context to the Arbitrum chain. Then the `check_sandbox_ownership` function checks the ownership of NFT in block `9,000,000`, but this time on a different chain - Arbitrum.

Currently, supported chains are Ethereum Sepolia and Optimism testnet.