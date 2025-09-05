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
    bytes32 internal fuzzingSeed;

    function setUp() public {
        // Proof has been generated with anvil, whereas we are checking against forge chain,
        // therefore blockhashes do not match.
        vm.roll(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER + 1);
    }

    function test_canVerifyFakeProof() public {
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_FAKE_SETTLE_BLOCK_HASH);
        vm.chainId(ProofFixtures.FIXED_SETTLE_CHAIN_ID);
        IProofVerifier verifier = new FakeProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.fakeProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }

    function test_canVerifyGroth16Proof() public {
        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
        vm.chainId(ProofFixtures.FIXED_SETTLE_CHAIN_ID);
        IProofVerifier verifier = new Groth16ProofVerifierUnderTest();
        (Proof memory proof, bytes32 journalHash) = ProofFixtures.groth16ProofFixture();

        verifier.verify(proof, journalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR);
    }

    function _randomBool() internal returns (bool) {
        fuzzingSeed = keccak256(abi.encode(fuzzingSeed));
        return uint256(fuzzingSeed) % 2 == 1;
    }

    // Enums can't be fuzzed https://github.com/foundry-rs/foundry/issues/871
    struct FuzzableProof {
        FuzzableSeal seal;
        bytes32 callGuestId;
        uint256 length;
        CallAssumptions callAssumptions;
    }

    struct FuzzableSeal {
        bytes4 verifierSelector;
        bytes32[8] seal;
        uint256 mode;
    }

    function _fromFuzzable(FuzzableProof memory proof) internal pure returns (Proof memory) {
        return Proof(_fromFuzzable(proof.seal), proof.callGuestId, proof.length, proof.callAssumptions);
    }

    function _fromFuzzable(FuzzableSeal memory seal) internal pure returns (Seal memory) {
        return Seal(seal.verifierSelector, seal.seal, ProofMode(seal.mode % uint256(type(ProofMode).max)));
    }

    function _arbitraryProof(Proof memory originalProof, Proof memory randomProof)
        internal
        returns (Proof memory, bytes32)
    {
        Proof memory arbitraryProof;
        arbitraryProof.seal.verifierSelector =
            _randomBool() ? randomProof.seal.verifierSelector : originalProof.seal.verifierSelector;
        arbitraryProof.seal.seal = _randomBool() ? randomProof.seal.seal : originalProof.seal.seal;
        arbitraryProof.seal.mode = _randomBool() ? randomProof.seal.mode : originalProof.seal.mode;
        arbitraryProof.callGuestId = _randomBool() ? randomProof.callGuestId : originalProof.callGuestId;
        arbitraryProof.length = originalProof.length; // Not actually verified
        arbitraryProof.callAssumptions.proverContractAddress = _randomBool()
            ? randomProof.callAssumptions.proverContractAddress
            : originalProof.callAssumptions.proverContractAddress;
        arbitraryProof.callAssumptions.functionSelector = _randomBool()
            ? randomProof.callAssumptions.functionSelector
            : originalProof.callAssumptions.functionSelector;
        arbitraryProof.callAssumptions.settleChainId =
            _randomBool() ? randomProof.callAssumptions.settleChainId : originalProof.callAssumptions.settleChainId;
        arbitraryProof.callAssumptions.settleBlockNumber = _randomBool()
            ? randomProof.callAssumptions.settleBlockNumber
            : originalProof.callAssumptions.settleBlockNumber;
        arbitraryProof.callAssumptions.settleBlockHash =
            _randomBool() ? randomProof.callAssumptions.settleBlockHash : originalProof.callAssumptions.settleBlockHash;
        return (
            arbitraryProof,
            ProofFixtures.journalHash(
                randomProof.callAssumptions, ProofFixtures.FIXED_OWNER, ProofFixtures.FIXED_BALANCE
            )
        );
    }

    function testFuzz_cannotVerifyManipulatedGroth16Proof(
        FuzzableProof calldata randomFuzzableProof,
        bytes32 _fuzzingSeed
    ) public {
        fuzzingSeed = _fuzzingSeed;
        Proof memory randomProof = _fromFuzzable(randomFuzzableProof);

        vm.setBlockhash(ProofFixtures.FIXED_SETTLE_BLOCK_NUMBER, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
        vm.chainId(ProofFixtures.FIXED_SETTLE_CHAIN_ID);
        IProofVerifier verifier = new Groth16ProofVerifierUnderTest();
        (Proof memory proof,) = ProofFixtures.groth16ProofFixture();

        (Proof memory arbitraryProof, bytes32 arbitraryJournalHash) = _arbitraryProof(proof, randomProof);
        vm.assume(keccak256(abi.encode(arbitraryProof)) != keccak256(abi.encode(proof)));

        try verifier.verify(
            arbitraryProof, arbitraryJournalHash, ProofFixtures.FIXED_PROVER_ADDRESS, ProofFixtures.FIXED_SELECTOR
        ) {
            revert("Should fail");
        } catch {}
    }
}

library ProofFixtures {
    bytes32 public constant FIXED_CALL_GUEST_ID =
        bytes32(0x012a42302dc53984d9973cf7442e0f0c206d0f1964fcb068c76461c21fd3df09);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 31_337;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 6;
    bytes32 public constant FIXED_GROTH16_SETTLE_BLOCK_HASH =
        bytes32(0x9f0d5475c5e2c723a95369ee1236c209ae08b3095012cfb2dce4f50b24645078);
    bytes32 public constant FIXED_FAKE_SETTLE_BLOCK_HASH =
        bytes32(0x30cad94f01d1d149488f012d183daa4af6c188fd717a9a49b96722951c0d1a7d);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x16707331d7c00a1ee539a5f78f146e80056b5da33b3a225bd927fc2ce2fdc3e7),
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
            bytes32(0x0f0266e9f115137fd1cf17607f8926d6670910edf369857535f47a353d90bfcb),
            bytes32(0x0a281dd953b4b16d0379373cdf6340d634cdf4c2d4a49b7895a007413a76dc7b),
            bytes32(0x17c05fae6f4751bbca7210358082d686651ace213ab1e87e84dd982c97907fa5),
            bytes32(0x18c1da7f1bad3a43d0d7de31866084b2188ca86c384623df6e2671d2826dbdb5),
            bytes32(0x09568a725fa357ebf143154fdc5f5460479d49e8235b641b838818d44a4c29ac),
            bytes32(0x19ca2376dff8eb308a85b6dea5be11f2b199a7d6671bd0f174fc7b3d0816453c),
            bytes32(0x09e14ea61635ce4f9aad47ab684c06d9b9a8dc8cddabfbbd26232d24c9576374),
            bytes32(0x25f92b78cef878a3c57ba8f01d5ea0c9d021e99b3da3b3f2b624fa1c54563769)
        ];

        Seal memory seal = Seal(Groth16VerifierSelector.STABLE_VERIFIER_SELECTOR, sealBytes, ProofMode.GROTH16);

        return generateProof(seal, ProofFixtures.FIXED_GROTH16_SETTLE_BLOCK_HASH);
    }

    function generateProof(Seal memory seal, bytes32 blockHash) private pure returns (Proof memory, bytes32) {
        CallAssumptions memory callAssumptions = CallAssumptions(
            FIXED_PROVER_ADDRESS, FIXED_SELECTOR, FIXED_SETTLE_CHAIN_ID, FIXED_SETTLE_BLOCK_NUMBER, blockHash
        );

        uint256 length = 0; // it is not used in verification, so can be set to 0

        Proof memory proof = Proof(seal, FIXED_CALL_GUEST_ID, length, callAssumptions);
        return (proof, journalHash(callAssumptions, FIXED_OWNER, FIXED_BALANCE));
    }

    function journalHash(CallAssumptions memory callAssumptions, address owner, uint256 balance)
        public
        pure
        returns (bytes32)
    {
        bytes memory journal = abi.encode(callAssumptions, ProofLib.emptyProof(), owner, balance);
        return sha256(journal);
    }
}
