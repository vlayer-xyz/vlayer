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
        bytes32(0x9eebe10658931b5001400fbe6133aaf7214c01a62af1b646a8685f7c49215606);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 1;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 4;
    bytes32 public constant FIXED_SETTLE_BLOCK_HASH =
        bytes32(0xa4b11124964d0edccf10a8f18771d5e3d8b28bbaa5dbd755f92b1ca22d749f5d);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x820c1cd3846493f80da25bbf75bef1a614992015e8ac9f27d14d5110af8cf832),
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
            bytes32(0x125ed7a937919cad6b5ea590279d27ef4281c726df4d0cb62eff59714405192b),
            bytes32(0x236969b16f7bfd911fe0e39bac39021b39e3e148f1c3ab5d39cbe4877212247a),
            bytes32(0x1bb91c289be59137327d0f1c1ad96063e447bc55fdf2bf0e6f913e611628ff87),
            bytes32(0x172ab5634c38a34b41b01dfce4cfe5f920d5f82b7f4fb3530c657bc07d9a88c0),
            bytes32(0x08052f9b048c6412ed1c63cbf1312c1feb85319cc283f0c741e6f64c5b654601),
            bytes32(0x1aa4f9029698b72449eaa592144b581fabbfe18f975bbd98b996704b072386ec),
            bytes32(0x2c1970ebb50e1916b46fb786afae4927dceab64b9b3642cc0c8c1f96543865e3),
            bytes32(0x06d5a27b5ecd7c03a3594f5d34983fd73fc3bb4c1fd9c9dad8917b61d5feee93)
        ];

        Seal memory seal = Seal(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, sealBytes, ProofMode.GROTH16);

        return generateProof(seal);
    }

    function generateProof(Seal memory seal) private pure returns (Proof memory, bytes32) {
        CallAssumptions memory callAssumptions = CallAssumptions(
            FIXED_PROVER_ADDRESS,
            FIXED_SELECTOR,
            FIXED_SETTLE_CHAIN_ID,
            FIXED_SETTLE_BLOCK_NUMBER,
            FIXED_SETTLE_BLOCK_HASH
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
