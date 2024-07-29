# Tests
vlayer supports automated testing for [Verifier](/advanced/verifier.html) and [Prover](/advanced/prover.html) contracts.

The vlayer test suite allows to simulate the entire flow from the generation of computation proofs to their verification in the on-chain contracts. 

If you have ever used [Foundry](https://book.getfoundry.sh/forge/tests) tests, it feels almost the same.

## Running tests
Test command searches for all the contract tests available in the working directory. 

Any contract with an external or public function that starts with `test` is considered to be a test. Usually, tests are be placed in the `test/` directory by convention and end with `.t.sol`.

To run all available tests, use the following command:
```sh
vlayer test
```

The above command starts local EVM nodes and runs the test code on them. 

## Test helpers 
To manipulate the state of the blockchain, as well as to test for specific reverts and events, Foundry introduced the concept of [cheatcodes](https://book.getfoundry.sh/forge/cheatcodes). These are special functions that allow to increase balances, impersonate specific accounts or simulate other behaviors in tests.

See below example of using `vm.prank()`:

```solidity
contract OwnerUpOnlyTest is Test {
    OwnerUpOnly upOnly;

    function testFail_IncrementAsNotOwner() public {
        vm.prank(address(0));
        upOnly.increment();
    }
}
```

`vm.prank(address(0))` sets `msg.sender` to the null address for the next call. This allows us to simulate a situation where the test fails because a non-owner account is calling the contract method.

## vlayer cheatcodes
vlayer also introduces a couple of new cheatcodes. See an example of how to use them below: 

```solidity
contract WebProverTest is Test {
    WebProver prover;
    WebVerifier verifier; 

    function test_mainProver() public {
        exectProving() // next call will run in the Prover
        uint venmoBalance = prover.main();
        Proof memory proof = getProofResult();
        verifier.main(proof, venmoBalance);

    }
}
```

In the above example we have two important cheatcodes: 
* `execProving()` sets the next call environment for the zkEVM environment, so that the proof of computation can be generated 
* `getProofResult()` gets the proof of the last call of `Prover` 

Using the above cheatcodes allows to test the whole proof journey: from setup & proving to verification. 