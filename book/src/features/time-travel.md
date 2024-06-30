# Time travel 

## Primer on blocks
The block number is a unique identifier assigned to each block in the Ethereum blockchain, starting from 0 for the genesis block and incrementing by one for each subsequent block. Smart contracts utilize block numbers to schedule events such as token releases, voting periods, and the start or end of auctions. By knowing the average block time, developers can estimate when these events will occur.

Unfortunately, direct access to historical blocks from within smart contracts is limited. This restriction means that smart contracts cannot easily reference past block data for decision-making or verification purposes.

## Handling historical data in vlayer 
To overcome the limitation of accessing historical blocks within smart contracts, we have introduced the `setBlockNumber(uint blockNo)` function, available in our `Prover` contracts. This function allows you to switch your next call context to the desired block number.

This allows you to aggregate and review data collected over multiple block numbers. 

## Example
Below is example Prover code which checks USDT balance of `msg.sender` at the begining and end of the specific period.

```solidity
contract USDTOwnership is Prover {
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