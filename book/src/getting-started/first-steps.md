# First Steps

## Off-chain execution

By convention, our off-chain smart contracts have a `.v.sol` extension. 
These contracts are written in [Solidity](https://soliditylang.org) and their bytecode is executed on vlayer zkEVM infrastructure. 

Off-chain execution allows cryptographic proofs to be generated. Once the proving is done, you can take the generated proof and then use it for on-chain verification.
Such a setup allows developers to use Solidity for generating zero-knowledge proofs. 

In addition, we introduce additional features such as [teleport]() (cross-chain verification) or [time machine]() (verifying data at a specific block number). 

All arguments passed to the contract functions are **private by default**.
In case you need to return some data (public inputs), just return them. 

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
