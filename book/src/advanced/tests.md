# Tests
<div class="feature-card feature-in-dev">
  <div class="title">
    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
    <path d="M8.57499 3.21665L1.51665 15C1.37113 15.252 1.29413 15.5377 1.29331 15.8288C1.2925 16.1198 1.3679 16.4059 1.51201 16.6588C1.65612 16.9116 1.86392 17.1223 2.11474 17.2699C2.36556 17.4174 2.65065 17.4968 2.94165 17.5H17.0583C17.3493 17.4968 17.6344 17.4174 17.8852 17.2699C18.136 17.1223 18.3439 16.9116 18.488 16.6588C18.6321 16.4059 18.7075 16.1198 18.7067 15.8288C18.7058 15.5377 18.6288 15.252 18.4833 15L11.425 3.21665C11.2764 2.97174 11.0673 2.76925 10.8176 2.62872C10.568 2.48819 10.2864 2.41437 9.99999 2.41437C9.71354 2.41437 9.43193 2.48819 9.18232 2.62872C8.93272 2.76925 8.72355 2.97174 8.57499 3.21665V3.21665Z" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 7.5V10.8333" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="M10 14.1667H10.0083" stroke="#FCA004" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    Actively in Development
  </div>
  <p>Our team is currently working on this feature. If you experience any bugs, please let us know <a href="https://discord.gg/JS6whdessP" target="_blank">on our Discord</a>. We appreciate your patience. </p>
</div>

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
