# Tests
Learn how to use automated testing in vlayer projects. Since vlayer execution is distributed across different environments, there are also different types of tests that we describe in this section.

## E2E tests 
End-to-end testing allows you to simulate the complete flow of proving and verifying claims on-chain. 

To run tests, use the following command: 
```sh
vlayer test
```

The above command sets up all the necessary components, runs the local EVM node, deploys the contracts, generates the proofs, and validates them.

## Contract tests
To run your smart contract unit tests, just use the command below:
```sh
forge test
```

The above command looks for the contract tests in your working directory. The [forge](https://book.getfoundry.sh/forge/) script will then run the tests, build and deploy your smart contracts.

## Test networks
You should test any contract code you write on a testnet before deploying to Mainnet. In order to deploy your contracts on Sepolia test network just run this command: 
```sh 
vlayer deploy --chainId 11155111
```

Above command will build and deploy all your vlayer `Prover` and `Verifier` contracts to the Sepolia public testnet (which has `chainId=11155111`).