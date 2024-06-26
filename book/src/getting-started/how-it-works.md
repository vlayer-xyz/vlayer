# How it works?

Vlayer contracts are just like regular on-chain contracts written in [Solidity](https://soliditylang.org). The main difference is the execution model, as bytecode is executed on the vlayer zkEVM infrastructure. 

## Features

Compared to regular contracts, vlayer smart contracts have additional features such as:
* **time travel** - ability to execute smart contracts on historaical data
* **teleport** - ability to execute smart contracts in the context of different chains
* access to **web content**
* access to **emails**

## Properties

vlayer smart contracts have following properties:
* verification - off-chain execution produces cryptographic proofs, which you can use  for on-chain settlement.
* built-in privacy - computation input is private and computations are executed off-chain, those details are never pubslihed on-chain
* no gas fees and transaction size limitation applies


![Off-chain execution simplified diagram](/images/offchain-execution.png)


Above diagram ilustrates this flow:
1. App sends private inputs to vlayer contract executed off-chain. 
1. Contract runs at zkEVM and returns proof of proper execution with public inputs
1. App sends & settles transaction to regular on-chain smart contract


> All arguments passed to the contract functions are **private by default**.
If you need to make certain arguments public outside the verifier contract, please return them from the verification function. 
