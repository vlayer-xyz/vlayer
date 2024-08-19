# Special Prover Variables
There are special Solidity variables and functions available in the global namespace. These entities mainly provide information about:
* The current state of the blockchain where the code is executed.
* Transaction details, including gas sent to the contract.

Since [Prover](/advanced/prover.html) contracts run in the [vlayer zkEVM environment](/appendix/architecture/prover.html), some variables are either not implemented or behave differently from those in standard EVM chains.

## Current block and chain
vlayer extends Solidity with features like [time traveling](/features/time-travel.html) between block numbers and [teleporting](/features/teleport.html) to other chains. This means that the values returned by `block.number` or `block.chainId` are affected by these features.

Initially, `block.number` returns the most recently mined block on the settlement chain. The settlement chain is specified either by a client JSON-RPC call or via `setChainId` function calls. However, a malicious Prover might attempt to manipulate the latest block number. To reduce this risk, it is recommended to prove data from specific block numbers instead.

## Hashes of older blocks
The `blockhash(uint blockNumber)` function returns the hash for the given blockNumber, but it only works for the 256 most recent blocks. Any block number outside this range returns 0. After the upcoming Pectra hardfork, we expect more block hashes to be available.

## vlayer-specific implementation
* `block.number`: Current block number, controlled by the vlayer Prover through [JSON-RPC calls](/appendix/api.html) or via [teleport](/features/teleport.html) and [time-travel](/features/time-travel.html) functions.
* `msg.sender`: Not usable; always points to the same address.
* `msg.sig`: Not usable; does not contain a valid signature.
* `block.chainid`: Current chain ID, controlled by the vlayer Prover through [JSON-RPC calls](/appendix/api.html) or via [teleport](/features/teleport.html) and [time-travel](/features/time-travel.html) functions.

## Unavailable variables
* `block.basefee`: Returns 0.
* `block.blobbasefee`: Returns 0.
* `block.coinbase(address payable)`: Returns the zero address `0x0`.
* `block.difficulty`: Returns 0.
* `block.gaslimit`: Returns 0.
* `block.prevrandao`: Returns 0.
* `msg.value`: Payable functionalities are unsupported; returns 0.

## Behaves the same as in Solidity
* `blockhash(uint blockNumber)`: Hash of the given block if `blockNumber` is one of the 256 most recent blocks; otherwise returns zero.
* `blobhash(uint index)`: Versioned hash of the `index`-th blob associated with the current transaction. Returns zero if no blob with the given index exists.
* `block.timestamp`: Current block timestamp in seconds since the Unix epoch.
* `gasleft`: Remaining gas.
* `msg.data`: Complete calldata.
* `tx.gasprice`: Gas price of the transaction.
* `tx.origin`: Sender of the transaction (full call chain).