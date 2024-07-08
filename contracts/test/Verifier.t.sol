// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {IRiscZeroVerifier, Receipt} from "risc0-ethereum/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0-ethereum/test/RiscZeroMockVerifier.sol";

import {Steel} from "vlayer-engine/Steel.sol";
import {Seal, SealLib} from "../src/Seal.sol";
import {Proof} from "../src/Proof.sol";

import {VerifierUnderTest} from "../src/Verifier.sol";

contract Prover {}

contract ExampleProver is Prover {
    function doSomething() public pure returns (uint256) {
        return 42;
    }
}

contract ExampleVerifier is VerifierUnderTest {
    address public immutable PROVER;

    bytes4 constant SIMPLE_PROVER_SELECTOR = ExampleProver.doSomething.selector;

    constructor() {
        PROVER = address(new ExampleProver());
    }

    function verifySomething(
        Proof calldata
    ) external onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR) returns (bool) {
        return true;
    }
}

contract Verifier_OnlyVerified_Modifier_Tests is Test {
    function test_verify_something() public {
        ExampleVerifier exampleVerifier = new ExampleVerifier();
        RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(bytes4(0));

        Steel.ExecutionCommitment memory commitment = Steel.ExecutionCommitment(
            exampleVerifier.PROVER(),
            ExampleVerifier.verifySomething.selector,
            block.number - 1,
            blockhash(block.number - 1)
        );

        bytes memory seal = mockVerifier
            .mockProve(
                exampleVerifier.GUEST_ID(),
                sha256(abi.encode(commitment))
            )
            .seal;

        exampleVerifier.setVerifier(mockVerifier);
        Proof memory proof = Proof(128, SealLib.encodeSeal(seal), commitment);
        exampleVerifier.verifySomething(proof);
    }
}
