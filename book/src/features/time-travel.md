# Time travel 

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

    function averageBalanceOf(address _owner) public returns (Proof, address, uint256) {
        uint256 totalBalance = 0;
        uint256 iterations = 0;

        for (uint256 blockNo = startingBlock; blockNo <= endingBlock; blockNo += step) {
            setBlock(blockNo);
            totalBalance += token.balanceOf(_owner); // USDC balance
            iterations += 1;
        }
        return (proof(), _owner, totalBalance / iterations);
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
> vlayer init --template simple-time-travel
> ```
> 
> This command will download all the necessary artefacts into your current directory (which must be empty). Make sure you have [Bun](https://bun.sh/) and [Foundry](https://book.getfoundry.sh/getting-started/installation) installed on your system.