# Header proof cache

## Storage proofs inside block
A storage proof prove that a pice of chain state (account or smart contract variable) belongs to a certain block.
To ensure a piece of state belonged to a certain chain at some block, it is not enough to prove it belongs to a block. We also need to prove that block itself belongs to the chain.
One way to do that is is to run solidity `blockhash` function. It returns a hash of a block for a given number. 
To perform a check we need to hash a block with certain state root and compare it with a result of the function.

This method is limited though, as it only works for 256 most recent blocks on given chain.
Therefore, we need another way to proof inclusion of older blocks in a chain.

## Block inclusion proofs
To proof inclusion of *historical blocks* (older than 256) in a chain, we use two separate proofs:
- one is the historical block 
- other is one of most recent 256 blocks.

To do an inclusion proof of two blocks in the same chain, we will need all the blocks between them. We order all of the blocks chronologically and iteratively check if every block  hash to value of `prevBlock` field in subsequent header.  

Unfortunately, this is a slow process, especially if the blocks are far away form each other on the time scale. Fortunately, there is a way to cache all proofs ahead of time. For this purpose, vlayer uses Block Proof Cache.

## Block Proof Cache

B
