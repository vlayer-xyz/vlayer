# Tests
Learn how to use automated testing in vlayer projects. 

Since vlayer execution is distributed across different environments, there are different types of tests to run. The following sections describe each of them. 

## E2E tests 
End-to-end testing allows you to simulate the entire flow from proving to verifying claims in the on-chain contracts. 

Use the following command to run tests:
```sh
vlayer test
```

The above command sets up all the necessary components, runs the local EVM node, deploys the contracts, generates the proofs, and validates them.

## Contract tests
To run your smart contract unit tests, simply use the command below:
```sh
forge test
```

The above command looks for all the contract tests in your working directory. The [forge](https://book.getfoundry.sh/forge/) script will then run the tests, build and deploy your smart contracts.

## Test networks
You should test any contract code you write on a testnet before deploying it to the mainnet. To deploy your contracts on the Sepolia testnet, simply run this command: 
```sh 
vlayer deploy --chainId 11155111
```

The above command will build and deploy all your vlayer `Prover` and `Verifier` contracts to the Sepolia public testnet (which has `chainId=11155111`).