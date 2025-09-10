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

contract PinnedProofVerifier_Tests is Test {
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
        bytes32(0x966a1250a0b661e418faff4e65c0a2408deb190ed7df88238fc9d9a86e0542c1);
    address public constant FIXED_PROVER_ADDRESS = address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0);
    bytes4 public constant FIXED_SELECTOR = PinnedSimpleProver.balance.selector;
    uint256 public constant FIXED_SETTLE_CHAIN_ID = 31_337;
    uint256 public constant FIXED_SETTLE_BLOCK_NUMBER = 6;
    bytes32 public constant FIXED_GROTH16_SETTLE_BLOCK_HASH =
        bytes32(0x66818135d98a7797c1b72523962d12bba0d3da6b27c3a366e1b505574783adb8);
    bytes32 public constant FIXED_FAKE_SETTLE_BLOCK_HASH =
        bytes32(0xa869a2aae2ef9053e9db49a0866751de61e136ed56fd7bf99e56bbfe28f994e8);

    address public constant FIXED_OWNER = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266);
    uint256 public constant FIXED_BALANCE = 10000000;

    function fakeProofFixture() public pure returns (Proof memory, bytes32) {
        bytes32[8] memory sealBytes = [
            bytes32(0x250618b5aaa645faa819d4a693235b407cd9de902797fad7d0c76a335dcca212),
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
            bytes32(0x2cb4dc6794a1c3f9a72cec7e53538cde77845d2e48e0c4514ded0e1115174e6a),
            bytes32(0x2572f745fe9801ecacdc66c74943060d5dfff70ca5f42138e542e8d8dfa39950),
            bytes32(0x2122e21ca20064a35508fc669d3062792a11f20de49d70bfbfe087edddcf6347),
            bytes32(0x0b7a5d33c2ddd2bbe023a9b651da47e47a7a8624023573b1e1e4c56362a7900e),
            bytes32(0x1201264c2caf3c86e8ae92a15aa85a03c5674ae54b8eac1ca65e25f3860ac847),
            bytes32(0x2d703b6eca2ba3a1fefdde3830646f84ed55c92b5d8d5dbb2cd0e003457cffb7),
            bytes32(0x2f75accf6e2b503bf6fe6fff2451afa3295c1fa4446d3f9c8e0ddfce1688eae9),
            bytes32(0x0d557eb01708158207bb36a08c1f94133b0028414b2aa0a3ef657a7bd07f215b)
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
