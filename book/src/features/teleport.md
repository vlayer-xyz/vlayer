# Teleport

## Ethereum ecosystem of chains
The Ethereum ecosystem is fragmented, consisting of various EVM chains such as Base, Arbitrum, Optimism, and many more. Developing applications that interact with multiple chains used to be challenging, but Teleport makes it easy.

## Teleporting betweens chains
`setChain(uint chainId, uint blockNo)` function, available in Prover contracts, allows to switch the context of execution to another chain (teleport).  It takes two arguments:
* `chainId`, which specifies the chain in the context of which the next function call will be executed
* `blockNo`, which is the block number of the given chain

## Example 
### Prover
The example below shows how to check USDC balances across three different chains.
Following tokens are passed to the constructor: 
```solidity
Erc20Token[] memory tokens = [
    Erc20Token(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48, 1, 20683110), // mainnet
    Erc20Token(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913, 8453, 19367633), // base
    Erc20Token(0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85, 10, 124962954) // optimism
];
```

```solidity
contract SimpleTeleportProver is Prover {
    Erc20Token[] public tokens;

    constructor(Erc20Token[] memory _tokens) {
        for (uint256 i = 0; i < _tokens.length; i++) {
            tokens.push(_tokens[i]);
        }
    }

    function crossChainBalanceOf(address _owner) public returns (Proof memory, address, uint256) {
        uint256 balance = 0;

        for (uint256 i = 0; i < tokens.length; i++) {
            setChain(tokens[i].chainId, tokens[i].blockNumber);
            balance += IERC20(tokens[i].addr).balanceOf(_owner);
        }

        return (proof(), _owner, balance);
    }
}
```

First, the call to `setChain(1, 20683110)` sets the chain to Ethereum mainnet (chainId = 1). Then, the ERC20 `balanceOf` function retrieves the USDC balance of `_owner` at block 20683110.

Next, `setChain(8453, 19367633)` switches the context to the Base chain. The `balanceOf` function then checks the balance at block 19367633, but this time on the Base chain.

Subsequent calls are handled by a for loop, which switches the context to the specified chains and block numbers accordingly.

### Verifier
After proving is complete, the generated proof and public inputs can be used for on-chain verification. 

```solidity
contract SimpleTravel is Verifier {
    address public prover;
    mapping(address => bool) public claimed;
    WhaleBadgeNFT public reward;

    constructor(address _prover, WhaleBadgeNFT _nft) {
        prover = _prover;
        reward = _nft;
    }

    function claim(Proof calldata, address claimer, uint256 crossChainBalance)
        public
        onlyVerified(prover, SimpleTravelProver.crossChainBalanceOf.selector)
    {
        require(!claimed[claimer], "Already claimed");

        if (crossChainBalance >= 10_000_000_000_00) { // 100 000 USD
            claimed[claimer] = true;
            reward.mint(claimer);
        }
    }
}
```
In this Verifier contract, the claim function lets users mint an NFT if their cross-chain USDC average balance is at least $100,000. The `onlyVerified` modifier ensures that the proof and public inputs (`claimer` and `crossChainBalance`) are correct.

If the proof or inputs are invalid, the transaction will revert, and the NFT will not be awarded.

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template simple-teleport
> ```
> 
> This command will download all the necessary artefacts into your current directory (which must be empty). Make sure you have [Bun](https://bun.sh/) and [Foundry](https://book.getfoundry.sh/getting-started/installation) installed on your system.

## Finality considerations
Finality, in the context of blockchains, is a point at which a transaction or block is fully confirmed and irreversible. When using vlayer `setChain` teleports, chain finality is an important factor to consider.

One should be aware that different chains may have different finality thresholds. For example, Ethereum Mainnet blocks are final after no more than about 12 minutes.

In the case of L2 chains, things are a bit more complicated. For example in case of optimistic rollup, like Optimism and Arbitrum, after L2 blocks are submitted to L1, there's a challenge period (often 7 days). If there is no evidence of an invalid state transition during this period, the L2 block is considered final.

Now consider teleporting to blocks that are not yet final in the destination chain. This can lead to situations where we are proving things that can be rolled back. It is important to include this risk in a protocol. The simplest way is to only teleport to blocks that are final and cannot be reorganized.