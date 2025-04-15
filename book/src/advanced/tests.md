# Tests

The prover and verifier contracts in vlayer are similar to regular smart contracts, allowing you to perform unit testing using your preferred smart contract testing framework.

vlayer introduces the `vlayer test` command, along with a couple of cheatcodes, which offers additional support for vlayer specific tests:
- Integration testing involving both the prover and the verifier
- Preparing data for the zkEVM proofs

This command uses Foundry's [Forge](https://book.getfoundry.sh/forge/tests) testing framework, so if you are familiar with it, you will find the process straightforward.

## Cheatcodes
To manipulate the blockchain state and test for specific reverts and events, Forge provides [cheatcodes](https://book.getfoundry.sh/forge/cheatcodes).

vlayer introduces additional cheatcodes:
- `callProver()`: Executes the **next call** within the vlayer zkEVM environment, generating a proof of computation accessible via `getProof`.
- `getProof()`: Retrieves the proof from the last call after using `callProver`.
- `preverifyEmail(string memory email) returns (UnverifiedEmail memory)`: Fetches the DNS for the RSA public key used to sign the email.

<div class="warning">

Similar to some other Forge cheatcodes, like [`prank`](https://book.getfoundry.sh/cheatcodes/prank) or [`expectEmit`](https://book.getfoundry.sh/cheatcodes/expect-emit), `callProver()`
must be used before the call to the prover function.

Also note, that majority of the cheatcodes are performing a call under the hood. This means, that if you use a cheatcode, like `console.log` between `callProver()` and the prover function call,  the proof will be
generated for the `console.log` call, not for the prover function call.

```solidity
    // Don't do this
    callProver();
    console.log("this will be proved, instead of prover.main()");
    uint venmoBalance = prover.main();
```

Another effect of the `callProver()` is that it effectively disables all the testing specific functions in the next call.
In general, `callProver()` **should only be used if you want to generate a proof** for the validation call, as it brings a noticeable overhead to the test.

</div>

### Differences against Forge

There are a few forge functionalities that are explicitly disabled in the vlayer tests:

- Having `setUp()` function in the test contract. Currently, every unit test needs to set up the environment on its own. It's still possible to create a separate function and call it at the beginning of each test.
- Watch mode
- Forking the blockchain

### Example Usage

```solidity
import {VTest} from "vlayer-0.1.0/testing/VTest.sol";

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
