# First Steps

## Off-chain execution
Vlayer contracts are just like regular on-chain contracts written in [Solidity](https://soliditylang.org). The main difference is the execution model, as bytecode is executed on the vlayer zkEVM infrastructure. 

Compared to regular contracts, vlayer smart contracts have additional features such as:
* [teleport]() (cross-chain verification)
* [time machine]() (verifying data at a specific block number)
* built-in privacy
* helpers for parsing web/email payloads 
* no gas fees and transaction size limitation 

Off-chain execution allows cryptographic proofs to be generated. Once the proving is done, you can take the generated proof and use it for on-chain settlement.

Below diagram ilustrates this flow:
1. App sends private inputs to vlayer contract executed off-chain. 
1. Contract runs at zkEVM and returns proof of proper execution with public inputs
1. App sends & settles transaction to regular on-chain smart contract

![Off-chain execution simplified diagram](/images/offchain-execution.png)

All arguments passed to the contract functions are **private by default**.
If you need public inputs, just return them.

Example use cases: 
- Alice can get airdrop by sending proof of some NFT ownership. She can do this without exposing her wallet address to the public. 
- Bob can recover his multisig wallet by sending [proof of email]() 
- Sarah can generate proof of web content returned by any HTTP server ([web proofs]())


## Initialisation

To initialise a vlayer project run:
```bash
$ vlayer init
```
which should be run inside a foundry-based project.

Above command will add all necessary dependencies and sample vlayer contracts.

## Testing

To run tests, one must first run:
```bash
$ anvil 
```
and in a new terminal session:

```bash
$ vlayer serve
``` 
