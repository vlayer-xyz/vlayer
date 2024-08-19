# Special Prover Variables
There are special Solidity variables and functions that exist in the global namespace. These entities are mainly used to provide information about:
* the current state of the blockchain in which the code is executed
* details about transaction and it's gas incoming into contract 

Since [Prover](/advanced/prover.html) contracts are executed in the [vlayer zkEVM environment](/appendix/architecture/prover.html), some variables are not implemented or behave in a completely different way than in regular EVM chains. 

## Current block and chain
vlayer allows to extend Solidity codebases with features like [time travelling](/features/time-travel.html) between block numbers or [teleporting](/features/teleport.html) to other destination chains. That means that value returned from `block.number` or `block.chainId` is affected by using these functionalities. 

Initially, `block.number` returns number of the most recently mined block on the settlement chain. The settlement chain is either specified by client JSON-RPC call or via `setChainId` function calls. However, it is important to note that malicious Prover may try to manipulate most recent number. To minimize this risks it is advised to prove data from specific block numbers instead. 

## vlayer specific implementation
* `block.number`: current block number, controlled either by vlayer prover [JSON-RPC call](/appendix/api.html) or [teleport](/features/teleport.html) / [time-travel](/features/time-travel.html) functions
* `msg.sender`: cannot be used as it always points to the same address
* `msg.sig`: cannot be used as it does not contain valid signature
* `block.chainid`: current chain id, controlled either by vlayer prover [JSON-RPC call](/appendix/api.html) or [teleport](/features/teleport.html) / [time-travel](/features/time-travel.html) functions

## Not available variables 
* `block.basefee`: returns 0
* `block.blobbasefee`: returns 0
* `block.coinbase(address payable)`: returns 0x0 zero address
* `block.difficulty`: returns 0
* `block.gaslimit `: returns 0
* `block.prevrandao`: returns 0
* `msg.value`: payable functionalities are not supported, returns 0

## Same behaviour as in Solidity
* `blockhash(uint blockNumber)`: hash of the given block when blocknumber is one of the 256 most recent blocks; otherwise returns zero
* `blobhash(uint index)`: versioned hash of the index-th blob associated with the current transaction. Returns zero if no blob with the given index exists.
* `block.timestamp`: current block timestamp as seconds since unix epoch
* `gasleft`: remaining gas
* `msg.data`: complete calldata
* `tx.gasprice`: gas price of the transaction
* `tx.origin`: sender of the transaction (full call chain)