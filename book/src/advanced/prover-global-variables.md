# Prover Global Variables
There are special Solidity variables and functions available in the global namespace. These entities mainly provide information about block, transaction and gas.

Since [Prover](/advanced/prover.html) contracts run in the [vlayer zkEVM environment](/appendix/architecture/prover.html), some variables are either not implemented or behave differently from those in standard EVM chains.

## Current Block and Chain
vlayer extends Solidity with features like [time traveling](/features/time-travel.html) between block numbers and [teleporting](/features/teleport.html) to other chains. As a result, the values returned by `block.number` or `block.chainId` are influenced by these features.

Initially, `block.number` returns one of the recently mined blocks in the settlement chain, known as the settlement block.

Generally, the prover will attempt to use the most recent block. However, proving takes time, and up to 256 blocks can be mined between the start of the proving process and the final settlement on-chain. Proofs older than 256 blocks will fail to verify. Additionally, a malicious prover might attempt to manipulate the last block number. Therefore, the guarantee is that the settlement block is not older than 256 blocks. In the future, the number of blocks allowed to be mined during proving may be significantly increased.

It is recommended to set `setBlock` to a specific block before making assertions.

Initially, `block.chainId` is set to the settlement chain ID, as specified in the JSON RPC call. Later, it can be changed using the `setChain()` function.

## Hashes of older blocks
The `blockhash(uint blockNumber)` function returns the hash for the given blockNumber, but it only works for the 256 most recent blocks. Any block number outside this range returns 0.

## vlayer specific implementation
* `block.number`: Current block number, as described in the [Current Block and Chain](#current-block-and-chain) section above.
* `block.chainid`: Current chain ID, as described in the [Current Block and Chain](#current-block-and-chain) section above.
* `blockhash(uint blockNumber)`: Hash of the given block if `blockNumber` is one of the 256 most recent blocks; otherwise returns zero.
* `block.timestamp`: Current block timestamp in seconds since the Unix epoch.
* `msg.sender`: Initially, set to a fixed address, c=have the same semantics as EVM after a call.

## Behaves the same as in Solidity
* `msg.data`: Complete calldata, passed by the prover.

## Unavailable variables
* `block.basefee`: Returns 0.
* `block.blobbasefee`: Returns 0.
* `block.coinbase(address payable)`: Returns the zero address `0x0`.
* `block.difficulty`: Returns 0.
* `block.gaslimit`: Returns 0.
* `block.prevrandao`: Returns 0.
* `msg.value`: Payable functionalities are unsupported; returns 0.
* `msg.sig`: Not usable; does not contain a valid signature.
* `tx.origin`: Sender of the transaction (full call chain).
* `blobhash(uint index)`: Versioned hash of the `index`-th blob associated with the current transaction. Returns zero if no blob with the given index exists.
* `gasleft`: Unused.
* `tx.gasprice`: Unused.

