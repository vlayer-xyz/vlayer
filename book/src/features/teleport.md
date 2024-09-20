# Teleport

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

## Ethereum ecosystem of chains
The Ethereum ecosystem is fragmented, consisting of various EVM chains such as Arbitrum, Optimism, Base, and many more. Developing applications that interact with multiple chains used to be challenging, but Teleport makes it easy.

## Teleporting betweens chains
`setChain(uint chainId, uint blockNo)` function, available in Prover contracts, allows to switch the context of execution to another chain (teleport).  It takes two arguments:
* `chainId`, which specifies the chain in the context of which the next function call will be executed
* `blockNo`, which is the block number of the given chain

## Example 

The example below shows how to check USDC balances across three different chains:

```solidity
contract CrossChainBalance is Prover {
    struct Erc20Token {
      address addr;
      uint256 chainId;
      uint256 blockNumber;
    }
    Erc20Token[] tokens = new Erc20Token[](3);

    constructor() {
        // Ethereum mainnet USDC
        tokens[0] = Erc20Token(0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48, 1, 20683110); 
        // Base USDC
        tokens[1] = Erc20Token(0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913, 8453, 19367633); 
        // Arbitrum USDC
        tokens[2] = Erc20Token(0x0b2C639c533813f4Aa9D7837CAf62653d097Ff85, 10, 124962954); 
    }

    function balanceOf(address _owner) public returns (address, uint256) {
        uint256 balance = 0;

        for (uint256 i = 0; i < tokens.length; i++) {
            setChain(tokens[i].chainId, tokens[i].blockNumber);
            balance += IERC20(tokens[i].addr).balanceOf(_owner);
        }

        return (_owner_, balance);
    }
}
```

First, the call to `setChain(1, 20683110)` sets the chain to Ethereum mainnet (chainId = 1). Then, the ERC20 `balanceOf` function retrieves the USDC balance of `_owner` at block 20683110.

Next, `setChain(8453, 19367633)` switches the context to the Base chain. The `balanceOf` function then checks the balance at block 19367633, but this time on the Base chain.

Subsequent calls are handled by a for loop, which switches the context to the specified chains and block numbers accordingly.

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template simple_teleport
> ```
> 
> This command will download all the necessary artefacts into your current directory (which must be empty). Make sure you have [Bun](https://bun.sh/) and [Foundry](https://book.getfoundry.sh/getting-started/installation) installed on your system.

## Finality considerations
Finality, in the context of blockchains, is a point at which a transaction or block is fully confirmed and irreversible. When using vlayer `setChain` teleports, chain finality is an important factor to consider.

One should be aware that different chains may have different finality thresholds. For example, Ethereum Mainnet blocks are final after no more than about 12 minutes.

In the case of L2 chains, things are a bit more complicated. For example in case of optimistic rollup, like Optimism and Arbitrum, after L2 blocks are submitted to L1, there's a challenge period (often 7 days). If there is no evidence of an invalid state transistion during this period, the L2 block is considered final.

Now consider teleporting to blocks that are not yet final in the destination chain. This can lead to situations where we are proving things that can be rolled back. It is important to include this risk in a protocol. The simplest way is to only teleport to blocks that are final and cannot be reorganized.