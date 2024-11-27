// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.21;

import {Test} from "forge-std-1.9.2/src/Test.sol";

import {Prover} from "../../src/Prover.sol";
import {Proof, ProofLib} from "../../src/Proof.sol";
import {IProofVerifier} from "../../src/proof_verifier/IProofVerifier.sol";
import {CallAssumptions} from "../../src/CallAssumptions.sol";
import {Seal, ProofMode} from "../../src/Seal.sol";

import {FakeProofVerifier} from "../../src/proof_verifier/FakeProofVerifier.sol";
import {Groth16ProofVerifier} from "../../src/proof_verifier/Groth16ProofVerifier.sol";

import {Groth16VerifierSelector} from "../helpers/Groth16VerifierSelector.sol";

// Proofs have been generated using SimpleProver from examples/simple
// All the pinned values have been obtained using the following instruction: https://github.com/vlayer-xyz/vlayer/pull/577#issuecomment-2355839549
interface PinnedSimpleProver {
    function balance(address _owner) external returns (Proof memory, address, uint256);
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
        bytes32(0x5dc07ff73c4a014e5801a307401b2a588b392ca5048e3c883dd6835530cec433);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 5;
    bytes32 public constant FIXED_SETTLE_BLOCK_HASH =
        bytes32(0x914ec0a71b8dccca1f0b3677c88a9723145160e03a265042b232394dfed0c975);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x4c555c8a3a3a6e8e9ab2c2907582b693f0aec66da6912b69435bbbcbeb671378),
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
            bytes32(0x1bc9902476684a405cfc1546d230068e0c24b383f56659c16a1e5e4bb8df1bf4),
            bytes32(0x2862caef0ed2c4815bb297b0664bfb26de1ac930b396a1a18a6b5432ac243fd4),
            bytes32(0x29b7f9e368b05ac9951655b7cfea9cfabc05294f610a49f6e59b08a7bfab1fc6),
            bytes32(0x264ed68434a140e840483692c08774de27ec9381a00d23f7ef00e416a93b14bd),
            bytes32(0x26eecaa574d7b5113e86c7eb561d7db709717603163de9f16eb3c154192ec209),
            bytes32(0x20101eafd91dc3da627c27af9a54f11cdeea59d8ee932ecfd3e5b9e6d86bbd6a),
            bytes32(0x003f431676099c8f63f12a7c9bee12abf80279bd8626cec774f67aaa4a1120aa),
            bytes32(0x227f66d3aa6032a000914a11eadbde056748c2e99d27c2f75234cd616c7b1e96)
        ];

        Seal memory seal = Seal(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, sealBytes, ProofMode.GROTH16);

        return generateProof(seal);
    }

    function generateProof(Seal memory seal) private pure returns (Proof memory, bytes32) {
        CallAssumptions memory callAssumptions =
            CallAssumptions(FIXED_PROVER_ADDRESS, FIXED_SELECTOR, FIXED_SETTLE_BLOCK_NUMBER, FIXED_SETTLE_BLOCK_HASH);

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
