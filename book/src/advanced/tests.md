# Tests
In many ways, the prover and verifier are just like regular smart contracts. Therefore, you can unit test with your favorite smart contract testing framework.

vlayer introduces the `vlayer test` command, which provides additional support for:
- testing prover functions that use `setBlock` and `setChain`
- integration testing of the prover working together with the verifier

The `vlayer test` command allows to simulate the entire flow from the generation of computation proofs, inside the zkEVM, to their verification in the on-chain contracts.

The command uses the Foundry's [forge](https://book.getfoundry.sh/forge/tests) tests, so if you have ever used it, you will feel right at home.

## Running tests
The test command searches for all the contract tests available in the working directory. 

Any contract with an external or public function that starts with `test` is considered to be a test. Usually, tests are be placed in the `test/` directory by convention and end with `.vt.sol`.

To run all available tests, use the following command:
```sh
vlayer test
```

The above command starts the local EVM nodes and executes the test code on them.

## Cheatcodes
To manipulate the state of the blockchain, as well as to test for specific reverts and events, Foundry introduced the concept of [cheatcodes](https://book.getfoundry.sh/forge/cheatcodes). These are special functions that can be used to increase balances, impersonate specific accounts or simulate other behaviors in tests.

vlayer introduces a couple of new cheatcodes:
* `execProving()` sets the next call environment for the zkEVM environment, so that the proof of computation can be generated 
* `getProofResult()` gets the proof of the last call of `Prover` 

See an example of how to use them below: 

```solidity
contract WebProverTest is VTest {
    WebProver prover;
    WebVerifier verifier; 

    function test_mainProver() public {
        callProver() // next call will run in the Prover
        uint venmo Balance = prover.main();
        Proof memory proof = getProof();
        verifier.main(proof, venmoBalance);
    }
}
```
Using the above cheatcodes allows testing the entire proof journey: from setting up and proving to verifying.