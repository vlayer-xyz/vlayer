// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.4/src/Test.sol";

import {Prover} from "../../src/Prover.sol";
import {Proof, ProofLib} from "../../src/Proof.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {CallAssumptions} from "../../src/CallAssumptions.sol";
import {Seal, ProofMode} from "../../src/Seal.sol";

import {Repository} from "../../src/Repository.sol";

import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";

import {Groth16VerifierSelector} from "../helpers/Groth16VerifierSelector.sol";

// Proofs have been generated using SimpleProver from examples/simple
// All the pinned values have been obtained using the following instruction: https://github.com/vlayer-xyz/vlayer/pull/577#issuecomment-2355839549
interface PinnedSimpleProver {
    function balance(address _owner) external returns (Proof memory, address, uint256);
}

contract FakeProofVerifierUnderTest is FakeProofVerifier {
    constructor() FakeProofVerifier(new Repository(address(this), address(this))) {
        IMAGE_ID_REPOSITORY.addImageIdSupport(ProofFixtures.FIXED_CALL_GUEST_ID);
    }
}

contract Groth16ProofVerifierUnderTest is Groth16ProofVerifier {
    constructor() Groth16ProofVerifier(new Repository(address(this), address(this))) {
        IMAGE_ID_REPOSITORY.addImageIdSupport(ProofFixtures.FIXED_CALL_GUEST_ID);
    }
}

contract PinnedProofVerifer_Tests is Test {
    function setUp() public {
        // Proof has been generated with anvil, whereas we are checking against forge chain,
        // therefore blockhashes do not match.
        vm.roll(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER + 1);
    }

    function test_canVerifyFakeProof() public {
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_FAKE_SETTLE_BLOCK_HASH);
        IProofVerifier verifier = new FakeProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.fakeProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }

    function test_canVerifyGroth16Proof() public {
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
        IProofVerifier verifier = new Groth16ProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.groth16ProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }
}

library ProofFixtures {
    bytes32 public constant FIXED_CALL_GUEST_ID =
        bytes32(0x48b3f1bbe8639486a88591ac8285d486b7eee373e6194f40e8310e3b31cbcb2c);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 31_337;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 6;
    bytes32 public constant FIXED_GROTH16_SETTLE_BLOCK_HASH =
        bytes32(0x686190a32cd4175beb26676c275fe31fe8ede32a3eb402235718ce08a2f0be37);
    bytes32 public constant FIXED_FAKE_SETTLE_BLOCK_HASH =
        bytes32(0xc22fc501b37d1a8cfa40dea699bf6dff9356dccb9d914be09fc39c1e3802c3a1);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0xd471a5bb30e787d84eae98c70f375ee2a674946eb47ace047eb748933c09bf9f),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000),
            bytes32(0x0000000000000000000000000000000000000000000000000000000000000000)
        ];

        Seal memory seal = Seal(bytes4(0xdeafbeef), sealBytes, ProofMode.FAKE);

        return generateProof(seal, ProofFixtures.FIXED_FAKE_SETTLE_BLOCK_HASH);
    }

    function groth16ProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x2f70448a3ecc532b5abe08657df85d8277fe340fc08507a8804166b6bf56e9b5),
            bytes32(0x09f47d1bef2a510bed7829f949ca2f75ab4d45a99d2e4bd9f717ff6251b17007),
            bytes32(0x077294343cc7fe14f8b17286bfd05c764dd662c8bcbc0d617ec2c866a6ae6808),
            bytes32(0x09b562299b7532db50999a1790bb062f15375b0d44964111b4c6b75c07ed15f1),
            bytes32(0x18dbcfcf84dbb509da769f4ad2da24a6597b929742953409f0b8a45c740702e3),
            bytes32(0x19436dec029157c3396542a41491199a57e5faa0da1a238d5e03ba283490d5c3),
            bytes32(0x0d309ab4096d78edd8586d7d2b0c326ec332dc7084e1781c3ca3ef4cb6bf9f7a),
            bytes32(0x2fb04107f123bbc92bced83e6a172164aaacd39838ebf68309a233e3c469bfa4)
        ];

        Seal memory seal = Seal(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, sealBytes, ProofMode.GROTH16);

        return generateProof(seal, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
    }

    function generateProof(Seal memory seal, bytes32 blockHash) private pure returns (Proof memory, bytes32) {
        CallAssumptions memory callAssumptions = CallAssumptions(
            FIXED_PROVER_ADDRESS,
            FIXED_SELECTOR,
            FIXED_SETTLE_CHAIN_ID,
            FIXED_SETTLE_BLOCK_NUMBER,
            blockHash
        );

        uint256 length = 0; // it is not used in verification, so can be set to 0

        Proof memory proof = Proof(seal, FIXED_CALL_GUEST_ID, length, callAssumptions);
        return (proof, journalHash(callAssumptions, FIXED_OWNER, FIXED_BALANCE));
    }

    function journalHash(CallAssumptions memory callAssumptions, address owner, uint256 balance)
        private
        pure
        returns (bytes32)
    {
        bytes memory journal = abi.encode(callAssumptions, ProofLib.emptyProof(), owner, balance);
        return sha256(journal);
    }
}
