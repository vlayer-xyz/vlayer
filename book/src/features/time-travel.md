# Time travel 

<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. In case of any bug please retry in 1-2 weeks. We appreciate your patience. </p>
</div>

## Primer on blocks
The block number is a unique identifier assigned to each block in the Ethereum blockchain, starting from 0 for the genesis block and incrementing by one for each subsequent block. Smart contracts utilize block numbers to schedule events such as token releases, voting periods, and the start or end of auctions. By knowing the average block time, developers can estimate when these events will occur.

## Access to historical data 
Unfortunately, direct access to historical state from within smart contracts is impossible. This restriction means that smart contracts cannot easily reference past accounts and smart contracts data for decision-making or verification purposes.

To overcome the limitation of accessing historical blocks within smart contracts, we have introduced the `setBlockNumber(uint blockNo)` function, available in our `Prover` contracts. This function allows you to switch your next function call context to the desired block number.

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

First, the call to the `setBlockNumber(15181682)` function sets the `Prover` context for the `15181682` block. This means that the next call to the `require_usdt_balance' function will read data in the context of the 15181682 block. As a result, the function will ensure that the caller owns at least $100 at this point in history.

Second call to `setBlockNumber(20175401)` sets the `Prover` context to block numbered `20175401`. The next call to `require_usdt_balance` checks if the caller owned at least $100, but this time in block `20175401`. Having less than $100 will result in an error (no proof will be generated).

The two `require_usdt_balance` calls return different results if the account balance has changed due to token transfers. 

> 💡  **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template time_travel_example
> ```
> 
> This command will download all necessary artifacts to your project.