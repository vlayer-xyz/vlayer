# Features
[Off-chain execution](/getting-started/first-steps.html#off-chain-execution) enables the extension of standard Solidity features. 
In this section we cover all the special powers that are available in vlayer contracts. 
## Teleport
As you may know, blockchain ecosystem is fragmented. There are EVM chains like: Arbitrum, Optimism, Polygon, Base and many more. Quering data from multiple chains silumatensly is tricky.  

That's why we decided to built Teleport functionality. It allows proving stuff from various chains:
```php
contract NftOwnership is VlayerProver {
  function main() public {
    // prove stuff from Ethereum Mainnet
    setChainId(1); 
    require(
      IERC721(BYAC_NFT_ADDR).balanceOf(msg.sender) > 0, "not owning any BYAC"
    );

    // prove stuff from Polygon
    setChainId(137);
    require(
      IERC721(SANDBOX_NFT_ADDR).balanceOf(msg.sender) > 0, "not owning any Sandbox"
    );
  }
}
```
Example use cases: 
- Prove your blockchain activity across multiple chains (claiming rewards, airdrops)
- Proof of reserve across multiple chains (interesting for DeFi protocols or DAOs)

## Time travel 
Block number is unique identifier for each block in the Ethereum blockchain, starting from 0 for the genesis block and incrementing by one for each subsequent block.
Smart contracts use block numbers to schedule events, such as token releases, voting periods, auctions start/end etc. By knowing the average block time, developers can estimate when these events will occur. 

Now imagine that you can prove your claims at any given block number:

```php
contract NftOwnership is VlayerProver {
  function main() public {
    // prove stuff from Ethereum Mainnet
    setChainId(1); 

    // prove on-chain stuff at certain time
    setBlockNumber(20175401); // Jun-26-2024 10:55:35 AM +UTC

    require(
      IERC20(USDT_ADDR).balanceOf(msg.sender) > 100000000, "must own at least $100"
    );
  }
}
```

Example use cases: 
- Prove that you had certain assets at specifc time stamps    
