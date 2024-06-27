# Time travel 
The block number is a unique identifier for each block in the Ethereum blockchain, starting at 0 for the genesis block and increasing by one for each subsequent block.

Smart contracts use block numbers to schedule events such as token releases, voting periods, the start and end of auctions, etc. By knowing the average block time, developers can estimate when these events will occur.

With that you may aggregate and check data gathered across multiple block numbers:

```solidity
contract NftOwnership is VlayerProver {
  function require_usdt_balance() {
    require(
      IERC20(USDT_ADDR).balanceOf(msg.sender) > 100000000, "must own at least $100"
    );
  }
  
  function main() public {
    setBlockNumber(15181682); 
    // Here we jump into 15181682 block, Jul-20-2022 08:40:21 PM +UTC 
    require_usdt_balance(); 

    setBlockNumber(20175401); 
    // Here we jump into 20175401 block, Jun-26-2024 10:55:35 AM +UTC 
    require_usdt_balance(); 
    
    return msg.sender;
  }
}
```

Example use cases: 
- Prove that you had certain assets at specifc time stamps    
