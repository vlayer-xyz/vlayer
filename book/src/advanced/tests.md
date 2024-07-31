# Tests

The prover and verifier contracts in vlayer are similar to regular smart contracts, allowing you to perform unit testing using your preferred smart contract testing framework.

vlayer introduces the `vlayer test` command, along with a couple of cheatcodes, which offers additional support for vlayer specific tests:
- Testing prover functions that utilize `setBlock` and `setChain`
- Integration testing involving both the prover and the verifier

This command uses Foundry's [Forge](https://book.getfoundry.sh/forge/tests) testing framework, so if you are familiar with it, you will find the process straightforward.

## Cheatcodes
To manipulate the blockchain state and test for specific reverts and events, Forge provides [cheatcodes](https://book.getfoundry.sh/forge/cheatcodes).

vlayer introduces additional cheatcodes:
- `callProver()`: Executes the next call within the vlayer zkEVM environment, generating a proof of computation accessible via `getProof`.
- `getProof()`: Retrieves the proof from the last call after using `callProver`.

### Example Usage

```solidity
contract WebProverTest is VTest {
    WebProver prover;
    WebVerifier verifier;

    function test_mainProver() public {
        callProver(); // The next call will execute in the Prover
        uint venmoBalance = prover.main();
        Proof memory proof = getProof();
        verifier.main(proof, venmoBalance);
    }
}
```

## Running Tests
The `vlayer test` command searches for all contract tests in the current working directory. 

Any contract with an external or public function beginning with `test` is recognized as a test. By convention, tests should be placed in the `test/` directory and should have a `.t.sol` extension and derive from `Test` contract.

vlayer specific tests are located in the `test/vlayer` directory and should derive from the `VTest` contract, which provides access to additional cheatcodes.

To run all available tests, use the following command:
```sh
vlayer test
```

This command runs both Forge tests and vlayer specific tests.
