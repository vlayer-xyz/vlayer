# Prover Global Variables

In the global namespace, Solidity provides special variables and functions that primarily offer information about blocks, transactions, and gas.

Since Prover contracts operate in the vlayer zkEVM environment, some variables are either not implemented or behave differently, compared to standard EVM chains.

## Current Block and Chain
vlayer extends Solidity with features like [time traveling](/features/time-travel.html) between block numbers and [teleporting](/features/teleport.html) to other chains. As a result, the values returned by `block.number` and `block.chainId` are influenced by these features.

Initially, `block.number` returns one of the recently mined blocks in the settlement chain, known as the settlement block.

Typically, the prover will use the most recent block. However, proving takes time, and up to 256 blocks can be mined between the start of the proving process and the final on-chain settlement. Proofs for blocks older than 256 blocks will fail to verify. Additionally, a malicious prover might try to manipulate the last block number. Therefore, the guarantee is that the settlement block is no more than 256 blocks old. In the future, the number of blocks allowed to be mined during proving may be significantly increased.

It is recommended to set `setBlock` to a specific block before making assertions.

Regarding `block.chainId`, initially it is set to the settlement chain ID, as specified in the JSON RPC call. Later, it can be changed using the `setChain()` function.

## Hashes of Older Blocks
The `blockhash(uint blockNumber)` function returns the hash for the given `blockNumber`, but it only works for the 256 most recent blocks. Any block number outside this range returns 0.

## vlayer-Specific Implementations
* `block.number`: The current block number, as described in the [Current Block and Chain](#current-block-and-chain) section.
* `block.chainid`: The current chain ID, as described in the [Current Block and Chain](#current-block-and-chain) section.
* `blockhash(uint blockNumber)`: Returns the hash of the given block if `blockNumber` is within the 256 most recent blocks; otherwise, it returns zero.
* `block.timestamp`: The current block timestamp in seconds since the Unix epoch.
* `msg.sender`: Initially set to a fixed address, it behaves like in standard EVM after a call.
* `block.prevrandao`: Returns pseudo-random uint, use with caution.
* `block.coinbase(address payable)`: Returns `0x0000000000000000000000000000000000000000`.

## Behaves the Same as in Solidity
* `msg.data`: The complete calldata, passed by the prover.

## Unavailable Variables
* `block.basefee`: Not usable.
* `block.blobbasefee`: Not usable.
* `block.difficulty`: Not usable.
* `block.gaslimit`: Returns 30000000.
* `msg.value`: Payable functionalities are unsupported; returns 0.
* `msg.sig`: Not usable; does not contain a valid signature.
* `tx.origin`: Sender of the transaction (full call chain).
* `blobhash(uint index)`: Not usable.
* `gasleft`: Unused.
* `tx.gasprice`: Unused.
