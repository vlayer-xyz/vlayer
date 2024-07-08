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

    function verifySomething(Proof calldata) external onlyVerified(PROVER, SIMPLE_PROVER_SELECTOR) returns (bool) {
        return true;
    }
}

contract Verifier_OnlyVerified_Modifier_Tests is Test {
    ExampleVerifier exampleVerifier = new ExampleVerifier();
    RiscZeroMockVerifier mockVerifier = new RiscZeroMockVerifier(bytes4(0));
    Steel.ExecutionCommitment commitment;

    function setUp() public {
        commitment = Steel.ExecutionCommitment(
            exampleVerifier.PROVER(), ExampleProver.doSomething.selector, block.number - 1, blockhash(block.number - 1)
        );
        exampleVerifier.setVerifier(mockVerifier);
    }

    function createProof() public view returns (Proof memory) {
        bytes memory seal = mockVerifier.mockProve(exampleVerifier.GUEST_ID(), sha256(abi.encode(commitment))).seal;
        return Proof(128, SealLib.encodeSeal(seal), commitment);
    }

    function test_verifySuccess() public {
        Proof memory proof = createProof();
        exampleVerifier.verifySomething(proof);
    }

    function test_invalidProver() public {
        commitment.startContractAddress = address(0x0000000000000000000000000000000000deadbeef);
        Proof memory proof = createProof();

        vm.expectRevert("Invalid prover");
        exampleVerifier.verifySomething(proof);
    }

    function test_invalidSelector() public {
        commitment.functionSelector = 0xdeadbeef;
        Proof memory proof = createProof();

        vm.expectRevert("Invalid selector");
        exampleVerifier.verifySomething(proof);
    }
}
