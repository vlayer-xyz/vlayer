# Time travel 
The block number is a unique identifier for each block in the Ethereum blockchain, starting at 0 for the genesis block and increasing by one for each subsequent block.

Smart contracts use block numbers to schedule events such as token releases, voting periods, the start/end of auctions, etc. By knowing the average block time, developers can estimate when these events will occur. Unfortunately, direct access to historical blocks from within smart contracts is limited. 

To overcome this limitation, we have introduced `setBlockNumber(uint blockNo)` function which is available in our Provers. It switches your next call context to the desired block number. 

This allows you to aggregate and review data collected over multiple block numbers:

```solidity
contract NftOwnership is Prover {
  function require_usdt_balance() {
    require(
      IERC20(USDT_ADDR).balanceOf(msg.sender) > 100000000, "must own at least $100"
    );
  }
  
  function main() public {
    setBlockNumber(15181682); 
    // Here we jump into 15181682 block, Jul-20-2022 08:40:21 PM +UTC 
    require_usdt_balance(); // checking balance in 15181682 block

    setBlockNumber(20175401); 
    // Here we jump into 20175401 block, Jun-26-2024 10:55:35 AM +UTC 
    require_usdt_balance(); // checking balance in 20175401 block
    
    return msg.sender;
  }
}
```
