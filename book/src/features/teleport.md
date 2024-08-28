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

> 💡 **Try it Now**
> 
> To run the above example on your computer, type the following command in your terminal:
> 
> ```bash
> vlayer init --template teleport_example
> ```
> 
> This command will download all necessary artifacts to your project.

## Finality considerations
Finality, in the context of blockchains, is a point at which a transaction or block is fully confirmed and irreversible. When using vlayer `setChainId` teleports, chain finality is an important factor to consider.

One should be aware that different chains may have different finality thresholds. For example, Ethereum Mainnet blocks are final after no more than about 12 minutes.

In the case of L2 chains, things are a bit more complicated. For example in case of optimistic rollup, like Optimism and Arbitrum, after L2 blocks are submitted to L1, there's a challenge period (often 7 days). If there is no evidence of an invalid state transistion during this period, the L2 block is finally considered as final.

Now consider teleporting to blocks that are not yet final in the destination chain. This can lead to situations where we are proving things that can be rolled back. It is important to include this risk in a protocol. The simplest way is to only teleport to blocks that are final and cannot be reorganized.