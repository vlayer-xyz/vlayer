// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std-1.9.2/src/Test.sol";

import {Prover} from "../../src/Prover.sol";
import {Proof} from "../../src/Proof.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {CallAssumptions} from "../../src/CallAssumptions.sol";
import {Seal, ProofMode} from "../../src/Seal.sol";

import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";

import {Groth16VerifierSelector} from "../helpers/Groth16VerifierSelector.sol";

// Proofs has been generated using SimpleProver from examples/simple
// All the pinned values have been obtained using the following instruction: https://github.com/vlayer-xyz/vlayer/pull/577#issuecomment-2355839549
contract PinnedSimpleProver is Prover {
    constructor() {}

    function sum(uint256 lhs, uint256 rhs) public pure returns (uint256) {
        return lhs + rhs;
    }
}

contract FakeProofVerifierUnderTest is FakeProofVerifier {
    constructor() {
        CALL_GUEST_ID = ProofFixtures.FIXED_CALL_GUEST_ID;
    }
}

contract Groth16ProofVerifierUnderTest is Groth16ProofVerifier {
    constructor() {
        CALL_GUEST_ID = ProofFixtures.FIXED_CALL_GUEST_ID;
    }
}

contract PinnedProofVerifer_Tests is Test {
    function setUp() public {
        // Proof has been generated with anvil, whereas we are checking against forge chain,
        // therefore blockhashes do not match.
        vm.roll(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER + 1);
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_SETTLE_BLOCK_HASH);
    }

    function test_canVerifyFakeProof() public {
        IProofVerifier verifier = new FakeProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.fakeProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }

    function test_canVerifyGroth16Proof() public {
        IProofVerifier verifier = new Groth16ProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.groth16ProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }
}

library ProofFixtures {
    bytes32 public constant FIXED_CALL_GUEST_ID =
        bytes32(0x17ae5657e365f3fb62f21ca9f8970452d3441ddaab415758e644bbd1587cd4da);
    address public constant FIXED_PROVER_ADDRESS = address(0x5FbDB2315678afecb367f032d93F642f64180aa3);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.sum.selector;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 2;
    bytes32 public constant FIXED_SETTLE_BLOCK_HASH =
        bytes32(0xcf196c6f636905301b1a44c84c380d289c71ef599c410bfa24d69ab03396adbe);

    uint256 public constant FIXED_LHS = 1;
    uint256 public constant FIXED_RHS = 2;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x933bd838d9a07b3b1ccc004c70e63711cd43c3e234de21bd105708b1b6922a52),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000)
        ];

        Seal memory seal = Seal(bytes4(0xdeafbeef), sealBytes, ProofMode.FAKE);

        return generateProof(seal);
    }

    function groth16ProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x0a703c1ac15f758ab379b36e692407a972797639b11326ecc32188b431080fb1),
            bytes32(0x27b0d99a9b52f6b0d64e193ed914867d967969d402170d17e73d91c2e7caeec5),
            bytes32(0x1f35d94edd6b0b09d74fcf47896c9f32c82f90180c136fe8d3be55ffcc7a1a1f),
            bytes32(0x13d5392f503062abad4887291d8876cf0d948239d65b7c2e4c07d9802e697cb2),
            bytes32(0x301c64d92932c40b13746d80a11996d49ed8f29095c1839f777b19ed670f45bb),
            bytes32(0x1c60b428f2c95cb58c7045dcec3a89fdb230a53c79f4a2a558025b3ba1fdf365),
            bytes32(0x03dd6a6f0595c24a44ef31a2b223afab6aa04556dee52af7c968d1b8981e8c92),
            bytes32(0x250cc2acb7e6d3a3d0360b91e3f84158f3c43e44012bd3cf164a1ad099d23f53)
        ];

        Seal memory seal = Seal(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, sealBytes, ProofMode.GROTH16);

        return generateProof(seal);
    }

    function generateProof(Seal memory seal) private pure returns (Proof memory, bytes32) {
        CallAssumptions memory callAssumptions =
            CallAssumptions(FIXED_PROVER_ADDRESS, FIXED_SELECTOR, FIXED_SETTLE_BLOCK_NUMBER, FIXED_SETTLE_BLOCK_HASH);

        uint256 length = 0; // it is not used in verification, so can be set to 0

        Proof memory proof = Proof(seal, FIXED_CALL_GUEST_ID, length, callAssumptions);
        return (proof, journalHash(callAssumptions, FIXED_LHS + FIXED_RHS));
    }

    function journalHash(CallAssumptions memory callAssumptions, uint256 proverResult) private pure returns (bytes32) {
        return sha256(abi.encode(callAssumptions, proverResult));
    }
}
