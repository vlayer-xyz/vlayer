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

## Access to historical data 
Unfortunately, direct access to the historical state from within smart contracts is not possible. 
Smart contracts only have access to the current state of the current block. 

To overcome this limitation, vlayer introduced the `setBlock(uint blockNo)` function, available in our `Prover` contracts. This function allows switching context of subsequent call to the desired block number.

This allows aggregating data from multiple blocks in a single call to a function. 

## Example
### Prover
The following is an example of Prover code that calculates the average USDC balance at specific block numbers.

```solidity
contract AverageBalance is Prover {
    IERC20 immutable token;
    uint256 immutable startingBlock;
    uint256 immutable endingBlock;
    uint256 immutable step;

    constructor() {
        token = IERC20(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48); // USDC 
        startingBlock = 6600000;
        endingBlock = 6700000;
        step = 10000;
    }

    function averageBalanceOf(address _owner) public returns (address, uint256) {
        uint256 totalBalance = 0;
        uint256 iterations = 0;

        for (uint256 blockNo = startingBlock; blockNo <= endingBlock; blockNo += step) {
            setChain(chainId, blockNo);
            totalBalance += token.balanceOf(_owner); // USDC balance
            iterations += 1;
        }
        return (_owner, totalBalance / iterations);
    }
}
```

First call to the `setBlock(blockNo)` function sets the `Prover` context for the `startingBlock` (`6600000` configured in the constructor). This means that the next call to the `token.balanceOf` function will read data in the context of the `6600000` block.

Next call to `setBlock()` sets the `Prover` context to block numbered `6610000` when step is configured to `10000`. The subsequent call to `token.balanceOf` checks again total balance, but this time in block `6610000`.

Each call to `token.balanceOf` can return different results if the account balance changes between blocks due to token transfers.

The for loop manages the balance checks, and the function’s final output is the average balance across multiple blocks.

### Verifier
After proving is complete, the generated proof and public inputs can be used for on-chain verification.

```solidity
contract AverageBalanceVerifier is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    HodlerBadgeNFT public reward;

    constructor(address _prover, HodlerBadgeNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256 average)
        public
        onlyVerified(prover, AverageBalance.averageBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (average >= 10_000_000) {
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
```

In this Verifier contract, the claim function allows users to mint an NFT if their average balance is at least 10,000,000. The `onlyVerified` modifier ensures the correctness of the proof and the provided public inputs (`claimer` and `average`).

If the proof is invalid or the public inputs are incorrect, the transaction will revert.

> 💡  **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template simple_time_travel
> ```
> 
> This command will download all the necessary artefacts into your current directory (which must be empty). Make sure you have [Bun](https://bun.sh/) and [Foundry](https://book.getfoundry.sh/getting-started/installation) installed on your system.