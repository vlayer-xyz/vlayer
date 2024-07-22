# Header proof cache

## Block inclusion proofs primer

### Storage proofs
A storage proof proves that a pice of chain state (account or smart contract variable) belongs to a certain block. To ensure a piece of state belonged to a certain chain, it is not enough to prove it belongs to a block. We also need to prove that block itself belongs to a chain.

### Recent and historical blocks
One way to do prove that block of certain hash belongs to a chain is to run solidity `blockhash(uint)` function. It returns a hash of a block for a given number. 
To perform a check we need to hash a block with certain state root and compare it with a result of the function.

However, this method is limited, as it only works for most recent 256 blocks on a given chain.
Therefore, we need another way to prove inclusion of older blocks in a chain. 

We use following naming in this document:
- *recent blocks* - any of most recent 256 blocks (relative to current block_no)
- *historical blocks* - blocks older than 256

### Naive block inclusion proofs
To proof inclusion of certain *historical blocks* in a chain, we will proof that:
1. Some *recent block* belong to a chain
2. Both *historical block* and *recent block* belong to the same chain

A naive way to prove inclusion proof of two blocks in the same chain, is to hash all subsequent blocks starting from *historical block* to *recent block* and verify that each blockhash equals subsequent block *prevHash* value.

Unfortunately, this is a slow process, especially if the blocks are far away form each other on the time scale. Fortunately, there is a way to cache all proofs ahead of time. For this purpose, vlayer uses Block Proof Cache.

